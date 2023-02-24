use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;
use log::{debug, error};
use pinky_swear::{Pinky, PinkySwear};
use serde_json::Value;

use crate::client::{web_os_network::WebOsSocketTvSend, WebSocketError};

use super::web_os_tv_request_communication::WebOsTvRequestCommunication;

pub struct WebOsTVRequestSender {
    write: Box<dyn WebOsSocketTvSend + Send>,
    command_id: Arc<Mutex<u8>>,
    ongoing_requests: Arc<Mutex<HashMap<u8, Pinky<Value>>>>,
}

impl WebOsTVRequestSender {
    pub fn new(
        web_socket_send: Box<dyn WebOsSocketTvSend + Send>,
        command_id: Arc<Mutex<u8>>,
        ongoing_requests: Arc<Mutex<HashMap<u8, Pinky<Value>>>>,
    ) -> WebOsTVRequestSender {
        WebOsTVRequestSender {
            write: web_socket_send,
            command_id,
            ongoing_requests,
        }
    }

    fn add_request(&self, pinky: Pinky<Value>, id: u8) {
        self.ongoing_requests.lock().unwrap().insert(id, pinky);
    }

    fn get_id(&self) -> u8 {
        let guard = self
            .command_id
            .lock()
            .expect("Could not lock next_command_id");

        *guard
    }

    fn next_id(&self) {
        let mut guard = self
            .command_id
            .lock()
            .expect("Could not lock next_command_id");
        let value = *guard;
        *guard = (1 + value) % 255;
    }
}

#[async_trait]
impl WebOsTvRequestCommunication for WebOsTVRequestSender {
    async fn send_json_request(
        &mut self,
        json: Value,
    ) -> Result<PinkySwear<Value>, WebSocketError> {
        let command_id = self.get_id();

        let mut json = json;
        json["id"] = Value::from(command_id);
        debug!("sending json {}", json);

        match serde_json::to_string(&json) {
            Ok(json_string) => {
                self.write.send_text(json_string).await?;
            }
            Err(e) => {
                error!("Unable to convert to string {:?}", e);
                return Err(WebSocketError::Continue);
            }
        }

        let (promise, pinky) = PinkySwear::<Value>::new();

        self.add_request(pinky, command_id);

        self.next_id();

        Ok(promise)
    }
}
