use futures::{stream::SplitStream, StreamExt};
use log::{debug, error};
use serde_json::json;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use crate::lg_command::{request_commands, LGCommandRequest};

use super::{
    WebOsSocketTvReceive, WebOsSocketTvSend, WebOsTvRequestCommunication, WebSocketConnection,
    WebSocketErrorAction,
};

pub struct InputPointerSocketConnection;

impl InputPointerSocketConnection {
    pub async fn try_to_connect(
        request_sender: &mut Box<dyn WebOsTvRequestCommunication>,
    ) -> Result<Box<dyn WebOsSocketTvSend>, WebSocketErrorAction> {
        let request =
            request_commands::web_os_services::GetPointerInputSocketUri.to_command_request();

        let promise = request_sender.send_json_request(json!(request)).await?;
        let json = promise.await;
        let address = json["payload"]["socketPath"].as_str().unwrap().to_string();

        let sender = WebSocketConnection::try_to_connect(&address).await?;

        let (sender, reciver) = sender.split();

        tokio::spawn(InputPointerSocketConnection::listener_loop(reciver));

        Ok(Box::new(sender))
    }

    async fn listener_loop(mut receiver: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>) {
        loop {
            let result = receiver.receive().await;

            match result {
                Ok(message) => {
                    debug!("Recived message on input pointer web socket: {message:?}");
                }
                Err(e) => {
                    error!("Error  {e:?} on recive message ");
                    return;
                }
            };
        }
    }
}
