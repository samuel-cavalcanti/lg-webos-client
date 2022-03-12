



use log::debug;
use pinky_swear::{ PinkySwear};
use serde_json::{json, Value};



use crate::client::handshake::do_the_handshake;
use crate::client::web_os_client_config::WebOsClientConfig;

use crate::client::web_os_request_sender::{connect_to_tv, init_sender_and_listener, WebOsTVRequestSender};

use crate::lg_command::{LGCommandRequest};



/// Client for interacting with TV
pub struct WebOsClient {
  tv_sender: WebOsTVRequestSender,
  pub key: Option<String>,
}

impl WebOsClient {

    pub async fn connect(config: WebOsClientConfig) -> WebOsClient {

        let ws_stream = connect_to_tv(&config.address).await;
        debug!("connected with TV");
        let mut sender = init_sender_and_listener(ws_stream);
        sender.key = config.key;
        do_the_handshake(&mut sender).await;

        debug!("WebSocket handshake has been successfully completed");

        let client = WebOsClient{
            key:sender.key.clone(),
            tv_sender:sender,

        };

        client
    }

    /// Sends single lg_command and waits for response
    pub async fn send_command_to_tv(&self, cmd: Box<dyn LGCommandRequest>) -> Value  {

        let promise = self.tv_sender.send_json(json!(cmd.to_command_request(0))).await;

        promise.await
    }



}

