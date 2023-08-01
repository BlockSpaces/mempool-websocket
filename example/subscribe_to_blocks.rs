/// Subscribes to new blocks from the mempool websocket api.

#[tokio::main]
async fn main() {
    let mut client = MempoolWebsocketClient::new("wss://mutinynet.com/api/v1/ws")
        .await
        .expect("Could not connect to mempool websocket.");

    client
        .subscribe_to_blocks()
        .await
        .expect("Getting blocks from mempool failed.");
}
