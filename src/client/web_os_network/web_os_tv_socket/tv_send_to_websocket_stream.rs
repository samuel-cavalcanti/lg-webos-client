use async_trait::async_trait;
use futures::SinkExt;
use log::debug;
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

use super::{WebOsSocketTvSend, WebSocketErrorAction};

#[async_trait]
impl WebOsSocketTvSend for WebSocketStream<MaybeTlsStream<TcpStream>> {
    async fn send_text(&mut self, text: String) -> Result<(), WebSocketErrorAction> {
        debug!("Sending PointerInputSender text: {}", text);
        let message = Message::Text(text);

        match self.send(message).await {
            Ok(ok) => Ok(ok),

            Err(e) => Err(WebSocketErrorAction::from(e)),
        }
    }
}
