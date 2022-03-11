use futures::{
    stream::{SplitSink, SplitStream},
     StreamExt,
};
use futures_util::{
    future::{join_all},
    SinkExt,
};
use log::debug;
use pinky_swear::{Pinky, PinkySwear};
use serde_json::Value;
use std::{
    cell::RefCell,
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tokio_tungstenite::{ MaybeTlsStream, WebSocketStream};

use crate::lg_command::{CommandResponse, LGCommandRequest};

use super::{
    utils::{get_handshake, process_messages_from_server},
    web_os_client_config::WebOsClientConfig,
};

/// Client for interacting with TV
pub struct WebosClient {
    write: RefCell<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>,
    next_command_id: Arc<Mutex<u8>>,
    ongoing_requests: Arc<Mutex<HashMap<u8, Pinky<CommandResponse>>>>,
    pub key: Option<String>,
}

impl WebosClient {
    /// Creates client connected to device with given address
    pub async fn new(config: WebOsClientConfig) -> Result<WebosClient, String> {
        let url = url::Url::parse(&config.address).expect("Could not parse given address");
        let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
        debug!("WebSocket handshake has been successfully completed");
        let (write, read) = ws_stream.split();
        WebosClient::from_stream_and_sink(read, write, config).await
    }

    /// Creates client using provided stream and sink
    pub async fn from_stream_and_sink(
        stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
        mut sink: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
        config: WebOsClientConfig,
    ) -> Result<WebosClient, String> {
        let next_command_id = Arc::from(Mutex::from(0));
        let ongoing_requests = Arc::from(Mutex::from(HashMap::new()));
        let requests_to_process = ongoing_requests.clone();
        let (registration_promise, registration_pinky) = PinkySwear::<Option<String>>::new();
        tokio::spawn(async move {
            /*
                in this new thread,  the server is going to wait new messages from TV and
                complete the promises when they arrived.
             */
            process_messages_from_server(stream, requests_to_process, registration_pinky).await
        });

        let mut handshake = get_handshake();
        // Check to see if the config has a key, if it does, add it to the handshake.
        if let Some(key) = config.key {
            handshake["payload"]["client-key"] = Value::from(key);
        }
        let formatted_handshake = format!("{}", handshake);
        sink.send(Message::text(formatted_handshake)).await.unwrap();
        let key = registration_promise.await;
        Ok(WebosClient {
            write: RefCell::new(sink),
            next_command_id,
            ongoing_requests,
            key,
        })
    }

    /// Sends single lg_command and waits for response
    pub async fn send_command(
        &self,
        cmd: Box<dyn LGCommandRequest>,
    ) -> Result<CommandResponse, String> {
        let (message, promise) = self.prepare_command_to_send(cmd);
        self.write.borrow_mut().send(message).await.unwrap();
        Ok(promise.await)
    }

    /// Sends mutliple commands and waits for responses
    pub async fn send_all_commands(
        &self,
        cmds: Vec<Box<dyn LGCommandRequest>>,
    ) -> Result<Vec<CommandResponse>, String> {
        let mut promises: Vec<PinkySwear<CommandResponse>> = vec![];

        let mut messages = Vec::new();

        for cmd in cmds {
            let (message, promise) = self.prepare_command_to_send(cmd);
            promises.push(promise);
            messages.push(Result::Ok(message));
        }

        let mut iter = futures_util::stream::iter(messages);
        self.write.borrow_mut().send_all(&mut iter).await.unwrap();
        Result::Ok(join_all(promises).await)
    }

    fn prepare_command_to_send(
        &self,
        cmd: Box<dyn LGCommandRequest>,
    ) -> (Message, PinkySwear<CommandResponse>) {
        let next_command_id = self.generate_next_id();
        let (promise, pinky) = PinkySwear::<CommandResponse>::new();

        self.ongoing_requests
            .lock()
            .unwrap()
            .insert(next_command_id, pinky);
        let message = Message::from(&cmd.to_command_request(next_command_id));
       
        (message, promise)
    }

    fn generate_next_id(&self) -> u8 {
        let mut guard = self
            .next_command_id
            .lock()
            .expect("Could not lock next_command_id");
        *guard += 1;
        (*guard).clone()
    }



}

