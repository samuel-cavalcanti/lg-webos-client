use async_trait::async_trait;
use pinky_swear::PinkySwear;
use serde_json::Value;

use crate::client::WebSocketErrorAction;

#[async_trait]
pub trait WebOsTvRequestCommunication {
    async fn send_json_request(
        &mut self,
        json: Value,
    ) -> Result<PinkySwear<Value>, WebSocketErrorAction>;
}
