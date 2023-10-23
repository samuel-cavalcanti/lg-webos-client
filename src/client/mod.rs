//! # Client Module
//!
//! Current, An WebOs Client must implement two traits, [`SendLgCommandRequest`] ,
//! [`SendPointerCommandRequest`], with [`SendPointerCommandRequest`], the Client can
//! control the WebOs's Mouse,
//! The [`SendLgCommandRequest`] is the send [`LGCommandRequest`] capability. You can do a lot
//! with it. See the all [`LGCommandRequest`] implementations in [`lg_command`].
//! The [`WebOsClient`] implements the two traits.
//!
mod web_os_client;
mod web_os_client_config;

mod web_os_network;

pub use web_os_client::WebOsClient;
pub use web_os_client_config::WebOsClientConfig;
pub use web_os_network::WebSocketError;

use crate::lg_command;
pub use async_trait::async_trait;
use lg_command::{pointer_input_commands::PointerInputCommand, LGCommandRequest};
use serde_json::Value;

/// The Trait Responsable to Send [`LGCommandRequest`] to TV.  
/// See All [`LGCommandRequest`] in [`lg_command`] module
#[async_trait]
pub trait SendLgCommandRequest {
    async fn send_lg_command_to_tv<R: LGCommandRequest>(
        &mut self,
        cmd: R,
    ) -> Result<Value, WebSocketError>;
}

#[async_trait]
/// The Trait Responsable to Send [`PointerInputCommand`] to TV.  
/// See All [`PointerInputCommand`] in [`lg_command`] module
pub trait SendPointerCommandRequest {
    async fn send_pointer_input_command_to_tv<R: PointerInputCommand>(
        &mut self,
        cmd: R,
    ) -> Result<(), WebSocketError>;
}
