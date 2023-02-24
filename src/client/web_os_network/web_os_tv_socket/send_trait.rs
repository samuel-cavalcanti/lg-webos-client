use async_trait::async_trait;

use super::error_action::WebSocketError;

#[async_trait]
pub trait WebOsSocketTvSend: Send {
    async fn send_text(&mut self, text: String) -> Result<(), WebSocketError>;
}
