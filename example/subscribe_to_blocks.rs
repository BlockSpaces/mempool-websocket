/// Subscribes to new blocks from the mempool websocket api.
use mempool_websocket::MempoolWebsocketClient;

#[tokio::main]
async fn main() {
    /// Instantiate a new websocket client.
    let mut client = MempoolWebsocketClient::new("wss://mutinynet.com/api/v1/ws")
        .await
        .expect("Could not connect to mempool websocket.");

    /// Optionally give a callback function when a block is received.
    client
        .on_receive_message(Box::new(|message| {
            println!("Received block!: {}", message.block.height);
        }))
        .await;

    /// Subscribe to blocks.
    client
        .subscribe_to_blocks()
        .await
        .expect("Getting blocks from mempool failed.");
}
