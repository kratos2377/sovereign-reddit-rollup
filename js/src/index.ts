import { createStandardRollup } from "@sovereign-sdk/web3";
import { type RuntimeCall } from "./types";
import { BasicSigner } from "./signer";

async function handleNewEvent(event: any): Promise<void> {
  console.log(event);
}

// Instantiate the rollup client
console.log("Initializing rollup client...");
const rollup = await createStandardRollup({
  context: {
    defaultTxDetails: {
      max_priority_fee_bips: 0,
      max_fee: "100000000",
      gas_limit: null,
      chain_id: 4321, // Note: Must match the chain id in constants.toml
    },
  },
});
console.log("Rollup client initialized.");

// Get the chain id that the `rollup` class fetched during initialization and configure it on the signer
let chain_hash = rollup.serializer.schema.chainHash;
// Private key taken from test-data/keys/minter_private_key.json
// We use this private key because we happen to know that it's pre-configured with
// tokens in the default genesis.json. We could generate a new key instead, but then we'd have to modify
// the genesis config to get tokens for it.
console.log("Initializing signer...");
let signer = await BasicSigner.fromPrivateKeyBytes(
  new Uint8Array([
    35, 110, 128, 203, 34, 44, 78, 208, 67, 27, 9, 59, 58, 197, 62, 106, 167,
    162, 39, 63, 225, 244, 53, 28, 211, 84, 152, 154, 130, 52, 50, 162,
  ]),
  chain_hash,
);
console.log("Signer initialized.");

// TODO: Use publicKeyToAddress to convert the public key to an address
let signer_address = "sov10d6chuh8vu86ltmt7qq4ec8lt25qyvr0cl3lg4mzs5llcfnx69m";

// Create a transaction to create a market
const now = Math.floor(Date.now() / 1000);
// let create_token_transaction: RuntimeCall = {
//   reddit_module: {
//     create_user: {
//       username: "ShadowMonarch"
//     },
//   },
// };


//Subreddit create ix
// let create_token_transaction: RuntimeCall = {
//   reddit_module: {
//     create_sub_reddit: {
//       user_address: "sov10d6chuh8vu86ltmt7qq4ec8lt25qyvr0cl3lg4mzs5llcfnx69m",
//       subname: "animals",
//       description: "details about animals"
//     },
//   },
// };


let create_token_transaction: RuntimeCall = {
  reddit_module: {
    create_post: {
      content: "This is some random panda",
      flair: "",
      subaddress: "sov1a6wpxvwqw3zhhhkehx7aple8h7xmrwapmzat7aca392s7f7ua0r",
      title: "Animal post 1"
    },
  },
};


const wait = (seconds: number) =>
  new Promise((resolve) => setTimeout(resolve, seconds * 1000));

console.log("Subscribing to rollup events...");
const subscription = rollup.subscribe("events", handleNewEvent); // Subscribe to events from the rollup
console.log("Subscribed.");

console.log("Sending create token transaction...");
await rollup.call(create_token_transaction, { signer }); // Send our transaction
console.log("Tx sent.");
await wait(2); // Wait for a couple seconds to get the events back before exit
// unsubscribe if needed

console.log(
  "Create token event should have been above. Unsubscribing to rollup events and exiting example script.",
);
subscription.unsubscribe();
