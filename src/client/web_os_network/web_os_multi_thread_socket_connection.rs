use futures::StreamExt;
use log::debug;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::client::web_os_network::web_os_request_response_cycle::{
    WebOsSocketResponseListener, WebOsTVRequestSender,
};

use crate::client::web_os_network::WebOsSocketTvReceive;

use crate::client::web_os_network::websocket_connection::WebSocketConnection;
use crate::client::{WebOsClientConfig, WebSocketError};

use super::connection::Connection;
use super::handshake::HandShake;

pub struct WebOsMultiThreadSocketConnection;

impl WebOsMultiThreadSocketConnection {
    pub async fn connect_to_tv(config: WebOsClientConfig) -> Result<Connection, WebSocketError> {
        let ws_stream = WebSocketConnection::try_to_connect(&config.address).await?;

        let (mut sender, mut web_socket_receive) = ws_stream.split();

        // let mut sender =write
        // let mut web_socket_receive = WebSocketTvReceive::new(read);

        let key =
            HandShake::do_the_handshake(&mut sender, &mut web_socket_receive, config.key).await?;

        debug!("WebSocket handshake has been successfully completed");

        let ongoing_requests = Arc::new(Mutex::new(HashMap::new()));

        let listener = WebOsSocketResponseListener::new(ongoing_requests.clone());
        let request_sender =
            WebOsTVRequestSender::new(Box::new(sender), Arc::new(Mutex::new(0)), ongoing_requests);

        tokio::spawn(async move {
            WebOsMultiThreadSocketConnection::listener_loop(listener, web_socket_receive).await;
        });

        Ok(Connection {
            request_sender: Box::new(request_sender),
            key,
        })
    }

    async fn listener_loop<Receive: WebOsSocketTvReceive>(
        listener: WebOsSocketResponseListener,
        mut web_socket_receive: Receive,
    ) {
        /*
           in this new thread,  the server is going to wait new messages from TV and
           complete the promises when they arrived.
        */
        debug!("listener has start");

        loop {
            match web_socket_receive.receive().await {
                Ok(json) => listener.receive_from_tv(json),
                Err(e) => {
                    if let WebSocketError::Fatal = e {
                        debug!("listener has stopped by Fatal error");
                        return;
                    }
                }
            }
        }
    }
}
