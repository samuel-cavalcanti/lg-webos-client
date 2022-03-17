use async_trait::async_trait;
use serde_json::Value;

use super::error_action::WebSocketErrorAction;
#[async_trait]
pub trait WebOsSocketTvReceive {
    async fn receive(&mut self) -> Result<Value, WebSocketErrorAction>;
}
