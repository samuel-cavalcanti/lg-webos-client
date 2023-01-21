use async_trait::async_trait;

use super::error_action::WebSocketErrorAction;

#[async_trait]
pub trait WebOsSocketTvSend: Send {
    async fn send_text(&mut self, text: String) -> Result<(), WebSocketErrorAction>;
}
