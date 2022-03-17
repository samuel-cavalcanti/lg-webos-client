use async_trait::async_trait;
use futures::{stream::SplitStream, StreamExt};
use log::error;
use serde_json::Value;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use super::{error_action::WebSocketErrorAction, receive_trait::WebOsSocketTvReceive};

pub struct WebSocketTvReceive {
    stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl WebSocketTvReceive {
    pub fn new(
        stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    ) -> WebSocketTvReceive {
        WebSocketTvReceive { stream }
    }
}

#[async_trait]
impl WebOsSocketTvReceive for WebSocketTvReceive {
    async fn receive(&mut self) -> Result<Value, WebSocketErrorAction> {
        // let mut stream = stream;

        match self.stream.next().await {
            Some(result_message) => match result_message {
                Ok(message) => match message.into_text() {
                    Ok(text_message) => {
                        return match serde_json::from_str::<Value>(&text_message) {
                            Ok(json) => Ok(json),
                            Err(e) => {
                                error!("Fail to convert to json {}", e.to_string());
                                Err(WebSocketErrorAction::Continue)
                            }
                        }
                    }
                    Err(e) => {
                        error!("Fail to consume message and convert to String");
                        Err(WebSocketErrorAction::from(e))
                    }
                },
                Err(e) => Err(WebSocketErrorAction::from(e)),
            },
            None => Err(WebSocketErrorAction::Continue),
        }
    }
}
