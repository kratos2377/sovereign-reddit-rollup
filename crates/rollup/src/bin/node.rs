//! This binary runs the rollup full node.

use anyhow::Context;
use clap::Parser;
use sov_modules_rollup_blueprint::logging::{
    default_rust_log_value, should_init_open_telemetry_exporter, OtelGuard,
};
use sov_modules_rollup_blueprint::FullNodeBlueprint;
use sov_modules_rollup_blueprint::Rollup;
use sov_rollup_interface::execution_mode::Native;
use sov_rollup_starter::da::DaService;
use sov_rollup_starter::rollup::StarterRollup;
use sov_rollup_starter::zkvm::{rollup_host_args, InnerZkvm};
use sov_stf_runner::processes::{RollupProverConfig, RollupProverConfigDiscriminants};
use sov_stf_runner::{from_toml_path, RollupConfig};
use std::env;
use std::path::PathBuf;
use std::str::FromStr;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

use sov_modules_api::Address;

const LOG_FILE_PREFIX: &str = "rollup.log";

#[cfg(all(feature = "mock_da", feature = "celestia_da"))]
compile_error!("Both mock_da and celestia_da are enabled, but only one should be.");

#[cfg(all(not(feature = "mock_da"), not(feature = "celestia_da")))]
compile_error!("Neither mock_da and celestia_da are enabled, but only one should be.");

#[cfg(all(feature = "risc0", feature = "sp1"))]
compile_error!("Both risc0 and sp1 are enabled, but only one should be.");

#[cfg(all(not(feature = "risc0"), not(feature = "sp1")))]
compile_error!("Neither risc0 and sp1 are enabled, but only one should be.");

#[cfg(all(feature = "mock_da", not(feature = "celestia_da")))]
const DA_STR: &str = "mock";
#[cfg(all(feature = "celestia_da", not(feature = "mock_da")))]
const DA_STR: &str = "celestia";

fn default_genesis_path() -> PathBuf {
    PathBuf::from_str(&format!("../../configs/{}/genesis.json", DA_STR))
        .expect("failed to construct default genesis path")
}

fn default_rollup_config_path() -> PathBuf {
    PathBuf::from_str(&format!("../../configs/{}/rollup.toml", DA_STR))
        .expect("failed to construct default genesis path")
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to the rollup config.
    #[arg(long, default_value = default_rollup_config_path().into_os_string())]
    rollup_config_path: PathBuf,

    /// The path to the genesis config.
    #[arg(long, default_value = default_genesis_path().into_os_string())]
    genesis_path: PathBuf,

    /// The optional path to the log file.
    #[arg(long, default_value = None)]
    log_dir: Option<String>,

    /// The optional path to the log file.
    #[arg(long, default_value_t = 9845)]
    metrics: u64,
}

fn init_logging(log_dir: Option<String>) -> (Option<WorkerGuard>, Option<OtelGuard>) {
    // Configuring filter
    let rust_log_value =
        env::var("RUST_LOG").unwrap_or_else(|_| default_rust_log_value().to_string());
    let env_filter = EnvFilter::from_str(&rust_log_value).unwrap();

    // Configuring layers.
    // Always on: stdout layer
    let stdout_layer = Some(fmt::layer().with_writer(std::io::stdout).boxed());

    // Option 1: Open Telemetry export.
    let (otel_guard, otel_layer) = if should_init_open_telemetry_exporter() {
        let otel = OtelGuard::new().unwrap();

        let otel_layer = otel
            .otel_tracing_layer()
            .boxed()
            .and_then(otel.otel_logging_layer());

        (Some(otel), Some(otel_layer.boxed()))
    } else {
        (None, None)
    };

    // Option 2: File appender
    let (file_guard, file_layer) = if let Some(path) = &log_dir {
        let file_appender = tracing_appender::rolling::daily(path, LOG_FILE_PREFIX);
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
        let file_layer = fmt::layer().with_ansi(false).with_writer(non_blocking);

        (Some(guard), Some(file_layer.boxed()))
    } else {
        (None, None)
    };

    // Initializing.
    tracing_subscriber::registry()
        .with(env_filter)
        .with(stdout_layer)
        .with(otel_layer)
        .with(file_layer)
        .init();

    print_information_about_logging(&rust_log_value, &log_dir);

    (file_guard, otel_guard)
}

fn print_information_about_logging(current_env_filter: &str, log_dir: &Option<String>) {
    tracing::info!(
        RUST_LOG = %current_env_filter,
        "Logging initialized; you can restart the node with a custom `RUST_LOG` environment variable to customize log filtering"
    );
    if !should_init_open_telemetry_exporter() {
        tracing::info!("Open Telemetry exporter was not enabled");
    }
    if let Some(log_dir) = log_dir {
        tracing::info!(dir = ?log_dir, prefix = LOG_FILE_PREFIX, "Logging to file has been enabled, besides stdout.")
    } else {
        tracing::info!("Logging to a file wasn't enabled, logging only to stdout.");
    }
}

#[tokio::main]
// Not returning result here, so error could be logged properly.
async fn main() {
    let args = Args::parse();

    println!("Args struct is: {:?}" , args);

    let guard = init_logging(args.log_dir);
    let prev_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        tracing_panic::panic_hook(panic_info);
        prev_hook(panic_info);
    }));

    let metrics_port = args.metrics;
    let address = format!("127.0.0.1:{}", metrics_port);
    prometheus_exporter::start(address.parse().unwrap())
        .expect("Could not start prometheus server");

    let prover_config_disc = parse_prover_config().expect("Malformed prover_config");
    tracing::info!(
        ?prover_config_disc,
        "Running demo rollup with prover config"
    );

    let prover_config =
        prover_config_disc.map(|config_disc| config_disc.into_config(rollup_host_args()));
    let rollup = new_rollup(args.genesis_path, args.rollup_config_path, prover_config)
        .await
        .expect("Couldn't start rollup");
    rollup.run().await.expect("Couldn't run rollup");
    drop(guard);
}

fn parse_prover_config() -> anyhow::Result<Option<RollupProverConfigDiscriminants>> {
    if let Some(value) = option_env!("SOV_PROVER_MODE") {
        tracing::warn!("SOV_PROVER_MODE is set to {}, but proving is not currently supported. Ignoring prover config.", value);
        Ok(None)
        // TODO: Re-enable proving once https://github.com/Sovereign-Labs/sovereign-sdk-wip/issues/2814 is resolved
        //
        // let config = std::str::FromStr::from_str(value).inspect_err(|&error| {
        //     tracing::error!(value, ?error, "Unknown `SOV_PROVER_MODE` value; aborting");
        // })?;
        // #[cfg(debug_assertions)]
        // {
        //     if config == RollupProverConfigDiscriminants::Prove {
        //         tracing::warn!(prover_config = ?config, "Given RollupProverConfig might cause slow rollup progression if not compiled in release mode.");
        //     }
        // }
        // Ok(Some(config))
    } else {
        Ok(None)
    }
}

async fn new_rollup(
    genesis_path: PathBuf,
    rollup_config_path: PathBuf,
    prover_config: Option<RollupProverConfig<InnerZkvm>>,
) -> Result<Rollup<StarterRollup<Native>, Native>, anyhow::Error> {
    tracing::info!(?rollup_config_path, "Starting rollup with config");

    let rollup_config: RollupConfig<Address, DaService> = from_toml_path(&rollup_config_path)
        .with_context(|| {
            format!(
                "Failed to read rollup configuration from {}",
                rollup_config_path.to_str().unwrap()
            )
        })?;

    let rollup = StarterRollup::default();

    rollup
        .create_new_rollup(&genesis_path, rollup_config, prover_config)
        .await
}
