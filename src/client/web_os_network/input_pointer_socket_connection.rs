use serde_json::json;

use crate::lg_command::{request_commands, LGCommandRequest};

use super::{WebOsSocketTvSend, WebOsTvRequestCommunication, WebSocketConnection};

pub struct InputPointerSocketConnection;

impl InputPointerSocketConnection {
    pub async fn try_to_connect(
        request_sender: &mut Box<dyn WebOsTvRequestCommunication>,
    ) -> Result<Box<dyn WebOsSocketTvSend>, String> {
        let request =
            request_commands::web_os_services::GetPointerInputSocketUri.to_command_request();

        match request_sender.send_json_request(json!(request)).await {
            Ok(promise) => {
                let json = promise.await;
                let address = json["payload"]["socketPath"].as_str().unwrap().to_string();

                let sender = WebSocketConnection::try_to_connect(&address).await?;

                Ok(Box::new(sender))
            }
            Err(e) => Err(format!(
                "Must be possible to get InputSocketUri, error: {e:?}"
            )),
        }
    }
}
