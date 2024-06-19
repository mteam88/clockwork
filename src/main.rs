use alloy::{
    network::EthereumWallet, primitives::{address, utils::parse_ether, Uint}, providers::{Provider, ProviderBuilder, WsConnect}, rpc::types::{BlockNumberOrTag, Filter}, signers::local::PrivateKeySigner, sol
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
        let TimeBasedExperience::SharesCreated { creator, referrer } = log.log_decode()?.inner.data;
        println!("Creator: {creator}, Referrer: {referrer}");

        println!("Buying shares...");
        let timefun = TimeBasedExperience::new(timefun_address, provider.clone());
        let tx = timefun.buyShares(creator, Uint::from(10)).value(parse_ether("0.008").unwrap());
        
        let receipt = tx.send().await?.with_required_confirmations(1).with_timeout(Some(std::time::Duration::from_secs(10))).watch().await?;

        println!("Transaction hash: {receipt}");
    }

    Ok(())
}