mod web_os_client;
mod web_os_client_config;

mod web_os_network;

pub use web_os_client::WebOsClient;
pub use web_os_client_config::WebOsClientConfig;
pub use web_os_network::WebSocketError;

pub use async_trait::async_trait;

use crate::lg_command::{pointer_input_commands::PointerInputCommand, LGCommandRequest};
use serde_json::Value;

#[async_trait]
pub trait SendLgCommandRequest {
    async fn send_lg_command_to_tv<R: LGCommandRequest>(
        &mut self,
        cmd: R,
    ) -> Result<Value, WebSocketError>;
}

#[async_trait]
pub trait SendPointerCommandRequest {
    async fn send_pointer_input_command_to_tv<R: PointerInputCommand>(
        &mut self,
        cmd: R,
    ) -> Result<(), WebSocketError>;
}
