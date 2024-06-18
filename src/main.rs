use alloy::{
    primitives::address,
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::{BlockNumberOrTag, Filter},
};
use eyre::Result;
use futures_util::stream::StreamExt;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let _private_key = dotenv::var("PRIVATE_KEY")?;

    // Set up the WS transport which is consumed by the RPC client.
    let rpc_url = dotenv::var("RPC_URL")?;

    // Create the provider.
    let ws = WsConnect::new(rpc_url);
    let provider = ProviderBuilder::new().on_ws(ws).await?;

    // Create a filter to watch for any SharesCreated events.
    let timefun_address = address!("428aeF7fB31E4E86162D62d4530a4dd7232D953D");
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
        println!("timefun event: {log:?}");
    }

    Ok(())
}