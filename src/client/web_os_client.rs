use log::debug;

use serde_json::{json, Value};

use crate::client::web_os_client_config::WebOsClientConfig;

use crate::client::web_os_network::{
    InputPointerSocketConnection, WebOsMultiThreadSocketConnection,
};
use crate::lg_command::pointer_input_commands::PointerInputCommand;
use crate::lg_command::LGCommandRequest;

use super::web_os_network::{WebOsSocketTvSend, WebOsTvRequestCommunication, WebSocketErrorAction};

/// Client for interacting with TV
pub struct WebOsClient {
    pub key: Option<String>,
    request_sender: Box<dyn WebOsTvRequestCommunication>,
    pointer_input_sender: Box<dyn WebOsSocketTvSend>,
}

impl WebOsClient {
    pub async fn connect(config: WebOsClientConfig) -> Result<WebOsClient, String> {
        let mut tv_connection = WebOsMultiThreadSocketConnection::connect_to_tv(config).await?;
        // let pointer_input_connection = WebOsPointerInputConnection::connect(&tv_connection.sender).await?;
        debug!("connected with TV");
        let pointer_input_sender =
            InputPointerSocketConnection::try_to_connect(&mut tv_connection.request_sender).await?;

        let client = WebOsClient {
            key: Some(tv_connection.key),
            request_sender: tv_connection.request_sender,

            pointer_input_sender,
        };

        Ok(client)
    }

    /// Sends single lg_command and waits for response
    pub async fn send_command_to_tv<R: LGCommandRequest>(
        &mut self,
        cmd: R,
    ) -> Result<Value, WebSocketErrorAction> {
        // let mut sender = &mut self.tv_sender;
        let request = cmd.to_command_request();
        let promise = self
            .request_sender
            .send_json_request(json!(request))
            .await?;

        Ok(promise.await)
    }

    pub async fn send_pointer_input_command_to_tv<R: PointerInputCommand>(
        &mut self,
        cmd: R,
    ) -> Result<(), WebSocketErrorAction> {
        self.pointer_input_sender
            .send_text(cmd.to_request_string())
            .await
    }
}
