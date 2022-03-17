use std::cell::RefCell;

use log::debug;

use serde_json::{json, Value};

use crate::client::web_os_client_config::WebOsClientConfig;

use crate::client::web_os_network::WebOsMultiThreadSocketConnection;
use crate::lg_command::LGCommandRequest;

use super::web_os_network::{WebOsTvRequestCommunication, WebSocketErrorAction};

/// Client for interacting with TV
pub struct WebOsClient {
    pub key: Option<String>,
    tv_sender: RefCell<Box<dyn WebOsTvRequestCommunication>>,
}

impl WebOsClient {
    pub async fn connect(config: WebOsClientConfig) -> Result<WebOsClient, String> {
        let tv_connection = WebOsMultiThreadSocketConnection::connect_to_tv(config).await?;
        // let pointer_input_connection = WebOsPointerInputConnection::connect(&tv_connection.sender).await?;
        debug!("connected with TV");

        let client = WebOsClient {
            key: Some(tv_connection.key),
            tv_sender: RefCell::new(tv_connection.sender),
        };

        Ok(client)
    }

    /// Sends single lg_command and waits for response
    pub async fn send_command_to_tv(
        &self,
        cmd: Box<dyn LGCommandRequest>,
    ) -> Result<Value, WebSocketErrorAction> {
        // let mut sender = &mut self.tv_sender;

        let promise = self
            .tv_sender
            .borrow_mut()
            .send_json_request(json!(cmd.to_command_request()))
            .await?;

        Ok(promise.await)
    }
}
