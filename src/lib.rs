pub mod mempool;

use futures_util::{SinkExt, StreamExt};
use mempool::{MempoolBlockResponse, MempoolMessage};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Error, Message},
    MaybeTlsStream, WebSocketStream,
};
use url::Url;

pub struct MempoolWebsocketClient {
    client: WebSocketStream<MaybeTlsStream<TcpStream>>,
    callback_fn: Option<MempoolMessageCallback>,
}

type MempoolMessageCallback = Box<dyn FnMut(MempoolBlockResponse) + Send>;

impl MempoolWebsocketClient {
    pub async fn new(url: &str) -> Result<MempoolWebsocketClient, Error> {
        let (socket, _) = connect_async(Url::parse(url).expect("Not a valid url.")).await?;

        Ok(MempoolWebsocketClient {
            client: socket,
            callback_fn: None,
        })
    }

    pub async fn subscribe_to_blocks(&mut self) -> Result<(), Error> {
        let blocks_msg = MempoolMessage::new("want".to_string(), vec!["blocks".to_string()]);
        let blocks_msg = serde_json::to_string(&blocks_msg).expect("Failed to serialize message.");

        let blocks_ws_msg = Message::Text(blocks_msg);
        self.client.send(blocks_ws_msg).await?;

        while let Some(mempool_response) = self.client.next().await {
            match mempool_response {
                Ok(Message::Text(response_json)) => {
                    let blocks: MempoolBlockResponse = serde_json::from_str(&response_json)
                        .expect("Failed to deserialize response.");
                    if let Some(ref mut callback) = &mut self.callback_fn {
                        callback(blocks);
                    }
                }
                Ok(_) => println!("Received non-text message."),
                Err(e) => {
                    println!("Error receiving message: {}", e);
                    break;
                }
            }
        }

        Ok(())
    }

    pub async fn on_receive_message(&mut self, callback: MempoolMessageCallback) {
        self.callback_fn = Some(callback);
    }

    pub async fn close(&mut self) -> Result<(), Error> {
        self.client.close(None).await?;
        Ok(())
    }
}
