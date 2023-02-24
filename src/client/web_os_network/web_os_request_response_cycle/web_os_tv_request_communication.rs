use async_trait::async_trait;
use pinky_swear::PinkySwear;
use serde_json::Value;

use crate::client::WebSocketError;

#[async_trait]
pub trait WebOsTvRequestCommunication: Send {
    async fn send_json_request(&mut self, json: Value)
        -> Result<PinkySwear<Value>, WebSocketError>;
}
