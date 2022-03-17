use async_trait::async_trait;
use futures::{stream::SplitSink, SinkExt};
use log::error;
use serde_json::Value;
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

use super::{send_trait::WebOsSocketTvSend, error_action::WebSocketErrorAction};


pub struct WebSocketTvSend {
    write: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
}

impl WebSocketTvSend {
    pub fn new(
        write: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    ) -> WebSocketTvSend {
        WebSocketTvSend { write }
    }
}

#[async_trait]
impl WebOsSocketTvSend for WebSocketTvSend {
    async fn send(&mut self, json: Value) -> Result<(), WebSocketErrorAction> {
        match serde_json::to_string(&json) {
            Ok(json_string) => {
                let message = Message::Text(json_string);

                match self.write.send(message).await {
                    Ok(ok) => Ok(ok),

                    Err(e) => Err(WebSocketErrorAction::from(e)),
                }
            }
            Err(e) => {
                error!("Unable to convert to string {:?}", e);
                Err(WebSocketErrorAction::Continue)
            }
        }
    }
}
