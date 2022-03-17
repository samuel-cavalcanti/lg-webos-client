use async_trait::async_trait;
use serde_json::Value;

use super::error_action::WebSocketErrorAction;

#[async_trait]
pub trait WebOsSocketTvSend {
    async fn send(&mut self, json: Value) -> Result<(), WebSocketErrorAction>;
}
