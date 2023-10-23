//! # WebOsClient
//!
//! This Client Connect to TV, though WebSocket connection.
//! [`WebOsClient`] struct holds the WebSocket Connetion.
//!  To create a Client you must know the Ip address of client, if the Operation system have
//!  mDns support than you can use: "lgwebostv.local"
//!
use async_trait::async_trait;
use log::debug;

use serde_json::{json, Value};

use crate::client::web_os_client_config::WebOsClientConfig;

use crate::client::web_os_network::{
    InputPointerSocketConnection, WebOsMultiThreadSocketConnection,
};
use crate::lg_command::pointer_input_commands::PointerInputCommand;
use crate::lg_command::LGCommandRequest;

use super::web_os_network::{WebOsSocketTvSend, WebOsTvRequestCommunication, WebSocketError};
use super::{SendLgCommandRequest, SendPointerCommandRequest};

/// Client for interacting with TV. This client only can used if the connection is successul,
pub struct WebOsClient {
    /// Every successul conection has a Token key
    pub key: String,
    /// A pointer to WebSocket Connection. This is the main connection with the Tv
    /// though this connection the client can send `LGCommandRequest`
    request_sender: Box<dyn WebOsTvRequestCommunication>,
    /// A pointer to WebSocket Connection, but this connection is responsable to control the WebOs
    /// pointer
    pointer_input_sender: Option<Box<dyn WebOsSocketTvSend>>,
}

impl WebOsClient {
    pub async fn connect(config: WebOsClientConfig) -> Result<WebOsClient, WebSocketError> {
        let tv_connection = WebOsMultiThreadSocketConnection::connect_to_tv(config).await?;
        debug!("connected with TV");

        let client = WebOsClient {
            key: tv_connection.key,
            request_sender: tv_connection.request_sender,

            pointer_input_sender: None,
        };

        Ok(client)
    }

    async fn try_to_connect_pointer_client(&mut self) -> Result<(), WebSocketError> {
        if self.pointer_input_sender.is_some() {
            return Ok(());
        }

        let pointer_input_sender =
            InputPointerSocketConnection::try_to_connect(&mut self.request_sender).await?;

        self.pointer_input_sender = Some(pointer_input_sender);

        Ok(())
    }
}

#[async_trait]
impl SendLgCommandRequest for WebOsClient {
    async fn send_lg_command_to_tv<R: LGCommandRequest>(
        &mut self,
        cmd: R,
    ) -> Result<Value, WebSocketError> {
        // let mut sender = &mut self.tv_sender;
        let request = cmd.to_command_request();
        let promise = self
            .request_sender
            .send_json_request(json!(request))
            .await?;

        Ok(promise.await)
    }
}

#[async_trait]
impl SendLgCommandRequest for &mut WebOsClient {
    async fn send_lg_command_to_tv<R: LGCommandRequest>(
        &mut self,
        cmd: R,
    ) -> Result<Value, WebSocketError> {
        // let mut sender = &mut self.tv_sender;
        let request = cmd.to_command_request();
        let promise = self
            .request_sender
            .send_json_request(json!(request))
            .await?;

        Ok(promise.await)
    }
}

#[async_trait]
impl SendPointerCommandRequest for WebOsClient {
    async fn send_pointer_input_command_to_tv<R: PointerInputCommand>(
        &mut self,
        cmd: R,
    ) -> Result<(), WebSocketError> {
        self.try_to_connect_pointer_client().await?;

        let text = cmd.to_request_string();
        match self.pointer_input_sender {
            Some(ref mut pointer_client) => pointer_client.send_text(text).await,
            None => Err(WebSocketError::Fatal),
        }
    }
}

#[async_trait]
impl SendPointerCommandRequest for &mut WebOsClient {
    async fn send_pointer_input_command_to_tv<R: PointerInputCommand>(
        &mut self,
        cmd: R,
    ) -> Result<(), WebSocketError> {
        self.try_to_connect_pointer_client().await?;

        let text = cmd.to_request_string();
        match self.pointer_input_sender {
            Some(ref mut pointer_client) => pointer_client.send_text(text).await,
            None => Err(WebSocketError::Fatal),
        }
    }
}
