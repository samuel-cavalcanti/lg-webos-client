use async_trait::async_trait;
use futures::{stream::SplitSink, SinkExt};

use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

use super::{error_action::WebSocketError, send_trait::WebOsSocketTvSend};

#[async_trait]
impl WebOsSocketTvSend for SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message> {
    async fn send_text(&mut self, text: String) -> Result<(), WebSocketError> {
        let message = Message::Text(text);

        match self.send(message).await {
            Ok(ok) => Ok(ok),

            Err(e) => Err(WebSocketError::from(e)),
        }
    }
}
