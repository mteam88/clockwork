use std::str::FromStr;

use alloy::{
    network::EthereumWallet, primitives::{utils::parse_units, Address, Uint}, providers::{Provider, ProviderBuilder, WsConnect}, rpc::types::{BlockNumberOrTag, Filter}, signers::local::PrivateKeySigner, sol
};
use eyre::Result;
use futures_util::stream::StreamExt;
use dotenv::dotenv;

// Codegen from ABI file to interact with the contract.
sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    TimeBasedExperience,
    "abi/TimeBasedExperience.json"
);

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let private_key = dotenv::var("PRIVATE_KEY")?;
    let signer: PrivateKeySigner = private_key.parse().expect("Invalid private key");
    let wallet = EthereumWallet::new(signer);

    // Set up the WS transport which is consumed by the RPC client.
    let rpc_url = dotenv::var("RPC_URL")?;

    // Create the provider.
    let ws = WsConnect::new(rpc_url);
    let provider = ProviderBuilder::new().with_recommended_fillers().wallet(wallet).on_ws(ws).await?;

    // Create a filter to watch for any SharesCreated events.
    let timefun_address = Address::from_str(dotenv::var("TIMEFUN_ADDRESS")?.as_str())?;
    let filter = Filter::new()
        .address(timefun_address)
        // By specifying an `event` or `event_signature` we listen for a specific event of the
        // contract. In this case the `SharesCreated` event.
        .event("SharesCreated(address,address)")
        .from_block(BlockNumberOrTag::Latest);

    // Subscribe to logs.
    let sub = provider.subscribe_logs(&filter).await?;
    let mut stream = sub.into_stream();

    println!("Listening for events...");

    while let Some(log) = stream.next().await {
        // println!("timefun event: {log:?}");
        let TimeBasedExperience::SharesCreated { creator, referrer } = log.log_decode()?.inner.data;
        println!("Creator: {creator}, Referrer: {referrer}");

        // Verify that the creator address has more than 0.0005 ETH.
        let balance = provider.get_balance(creator).await?;
        if balance < parse_units("0.0005", "ether").unwrap().into() {
            println!("Creator has less than 0.0005 ETH, skipping");
            continue;
        }

        let timefun = TimeBasedExperience::new(timefun_address, provider.clone());

        println!("Buying minutes...");
        let minutes = dotenv::var("NUM_MINUTES")?.parse::<u64>().expect("Invalid minutes");

        let TimeBasedExperience::totalCostWithFeesReturn { _0: cost_of_minutes } = timefun.totalCostWithFees(creator, Uint::from(minutes), true).call().await.expect("Failed to estimate cost");

        let tx = timefun.buyShares(creator, Uint::from(minutes)).value(cost_of_minutes);
        
        let receipt = tx.send().await?.with_required_confirmations(1).with_timeout(Some(std::time::Duration::from_secs(10))).watch().await.expect("Failed to send transaction");

        println!("Transaction hash: {receipt}");
    }

    Ok(())
}