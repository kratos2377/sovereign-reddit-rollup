#![deny(missing_docs)]
//! StarterRollup provides a minimal self-contained rollup implementation

use async_trait::async_trait;
use sov_db::ledger_db::LedgerDb;
use sov_db::storage_manager::NativeStorageManager;
use sov_mock_zkvm::MockCodeCommitment;
use sov_modules_api::default_spec::DefaultSpec;
use sov_modules_api::rest::StateUpdateReceiver;
use sov_modules_api::ZkVerifier;
use sov_modules_api::{CryptoSpec, Spec};
use sov_modules_rollup_blueprint::pluggable_traits::PluggableSpec;
use sov_modules_rollup_blueprint::proof_sender::SovApiProofSender;
use sov_modules_rollup_blueprint::{FullNodeBlueprint, RollupBlueprint, SequencerCreationReceipt};

use sov_rollup_interface::execution_mode::{ExecutionMode, Native};
use sov_rollup_interface::node::SyncStatus;
use sov_rollup_interface::zk::aggregated_proof::CodeCommitment;
use sov_sequencer::ProofBlobSender;
use sov_state::Storage;
use sov_state::{DefaultStorageSpec, ProverStorage};
use sov_stf_runner::processes::{ParallelProverService, ProverService, RollupProverConfig};
use sov_stf_runner::RollupConfig;
use std::sync::Arc;
use stf_starter::Runtime;
use tokio::sync::watch;
use sov_modules_api::common::Base58Address;
use sov_modules_api::configurable_spec::ConfigurableSpec;
use crate::da::{new_da_service, new_verifier, DaService, DaSpec};
use crate::zkvm::{get_outer_vm, InnerZkvm, InnerZkvmHost, OuterZkvm};

type SolanaSpec<Da, InnerZkvm, OuterZkvm, Mode> =
    ConfigurableSpec<Da, InnerZkvm, OuterZkvm, Base58Address, Mode>;

/// Starter rollup implementation.
#[derive(Default)]
pub struct StarterRollup<M> {
    phantom: std::marker::PhantomData<M>,
}

/// This is the place where all the rollup components come together, and
/// they can be easily swapped with alternative implementations as needed.
impl RollupBlueprint<Native> for StarterRollup<Native>
where
   SolanaSpec<DaSpec, InnerZkvm, OuterZkvm, Native>: PluggableSpec,
{
    type Spec = SolanaSpec<DaSpec, InnerZkvm, OuterZkvm, Native>;
    type Runtime = Runtime<Self::Spec>;
}

#[async_trait]
impl FullNodeBlueprint<Native> for StarterRollup<Native> {
    type DaService = DaService;

    type StorageManager = NativeStorageManager<
        DaSpec,
        ProverStorage<DefaultStorageSpec<<<Self::Spec as Spec>::CryptoSpec as CryptoSpec>::Hasher>>,
    >;

    type ProverService = ParallelProverService<
        <Self::Spec as Spec>::Address,
        <<Self::Spec as Spec>::Storage as Storage>::Root,
        <<Self::Spec as Spec>::Storage as Storage>::Witness,
        Self::DaService,
        <Self::Spec as Spec>::InnerZkvm,
        <Self::Spec as Spec>::OuterZkvm,
    >;

    type ProofSender = SovApiProofSender<Self::Spec>;

    fn create_outer_code_commitment(
        &self,
    ) -> <<Self::ProverService as ProverService>::Verifier as ZkVerifier>::CodeCommitment {
        MockCodeCommitment::default()
    }

    async fn create_endpoints(
        &self,
        state_update_receiver: StateUpdateReceiver<<Self::Spec as Spec>::Storage>,
        sync_status_receiver: watch::Receiver<SyncStatus>,
        shutdown_receiver: watch::Receiver<()>,
        ledger_db: &LedgerDb,
        sequencer: &SequencerCreationReceipt<Self::Spec>,
        _da_service: &Self::DaService,
        rollup_config: &RollupConfig<<Self::Spec as Spec>::Address, Self::DaService>,
    ) -> anyhow::Result<sov_modules_api::NodeEndpoints> {
        sov_modules_rollup_blueprint::register_endpoints::<Self, _>(
            state_update_receiver.clone(),
            sync_status_receiver,
            shutdown_receiver,
            ledger_db,
            sequencer,
            rollup_config,
        )
        .await
    }

    async fn create_da_service(
        &self,
        rollup_config: &RollupConfig<<Self::Spec as Spec>::Address, Self::DaService>,
        shutdown_receiver: tokio::sync::watch::Receiver<()>,
    ) -> Self::DaService {
        new_da_service::<Self::Spec>(rollup_config, shutdown_receiver).await
    }

    async fn create_prover_service(
        &self,
        prover_config: RollupProverConfig<<Self::Spec as Spec>::InnerZkvm>,
        rollup_config: &RollupConfig<<Self::Spec as Spec>::Address, Self::DaService>,
        _da_service: &Self::DaService,
    ) -> Self::ProverService {
        let (elf, prover_config_disc) = prover_config.split();
        let inner_vm = InnerZkvmHost::new(*elf);
        let outer_vm = get_outer_vm();
        let da_verifier = new_verifier();

        ParallelProverService::new_with_default_workers(
            inner_vm,
            outer_vm,
            da_verifier,
            prover_config_disc,
            CodeCommitment::default(),
            rollup_config.proof_manager.prover_address,
        )
    }

    fn create_storage_manager(
        &self,
        rollup_config: &RollupConfig<<Self::Spec as Spec>::Address, Self::DaService>,
    ) -> anyhow::Result<Self::StorageManager> {
        NativeStorageManager::new(&rollup_config.storage.path)
    }

    fn create_proof_sender(
        &self,
        _rollup_config: &RollupConfig<<Self::Spec as Spec>::Address, Self::DaService>,
        sequence_number_provider: Arc<dyn ProofBlobSender>,
    ) -> anyhow::Result<Self::ProofSender> {
        Ok(Self::ProofSender::new(sequence_number_provider))
    }
}

impl sov_modules_rollup_blueprint::WalletBlueprint<Native> for StarterRollup<Native> {}