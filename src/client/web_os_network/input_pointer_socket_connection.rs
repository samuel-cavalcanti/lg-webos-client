use crate::lg_command::{request_commands, LGCommandRequest};
use futures::StreamExt;
use log::{debug, error};
use serde_json::json;

use super::{
    WebOsSocketTvReceive, WebOsSocketTvSend, WebOsTvRequestCommunication, WebSocketConnection,
    WebSocketError,
};

pub struct InputPointerSocketConnection;

impl InputPointerSocketConnection {
    pub async fn try_to_connect(
        request_sender: &mut Box<dyn WebOsTvRequestCommunication>,
    ) -> Result<Box<dyn WebOsSocketTvSend>, WebSocketError> {
        let address = InputPointerSocketConnection::get_socket_path(request_sender).await?;
        let sender = WebSocketConnection::try_to_connect(&address).await?;

        let (sender, reciver) = sender.split();

        tokio::spawn(async {
            let mut reciver = reciver;
            loop {
                InputPointerSocketConnection::listener_loop(&mut reciver)
                    .await
                    .unwrap()
            }
        });

        Ok(Box::new(sender))
    }

    async fn get_socket_path(
        request_sender: &mut Box<dyn WebOsTvRequestCommunication>,
    ) -> Result<String, WebSocketError> {
        let request =
            request_commands::web_os_services::GetPointerInputSocketUri.to_command_request();

        let promise = request_sender.send_json_request(json!(request)).await?;
        let json = promise.await;
        let address = match json["payload"]["socketPath"].as_str() {
            Some(text) => text.to_string(),
            None => {
                error!("Socket path not found in JSON: {json:?}");
                return Err(WebSocketError::Fatal);
            }
        };

        Ok(address)
    }

    async fn listener_loop<R: WebOsSocketTvReceive>(
        receiver: &mut R,
    ) -> Result<(), WebSocketError> {
        let result = receiver.receive().await;

        match result {
            Ok(message) => {
                debug!("Recived message on input pointer web socket: {message:?}");
                Ok(())
            }
            Err(e) => match e {
                WebSocketError::Fatal => {
                    error!("Recived Fatal error on pointer listener loop: {e:?}");
                    Err(e)
                }
                WebSocketError::Continue => {
                    debug!("Recived continue error on pointer listener loop: {e:?}");
                    Ok(())
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {

    use async_trait::async_trait;
    use pinky_swear::PinkySwear;
    use serde_json::{json, Value};

    use super::*;
    use super::{WebOsSocketTvReceive, WebOsTvRequestCommunication, WebSocketError};

    enum TestCases {
        FatalError,
        ContinueError,
        CorrectJson,
        WrongJson,
    }

    struct MockWebOsSocketTvReceive {
        case: TestCases,
    }

    #[async_trait]
    impl WebOsSocketTvReceive for MockWebOsSocketTvReceive {
        async fn receive(&mut self) -> Result<Value, WebSocketError> {
            match self.case {
                TestCases::ContinueError => Err(WebSocketError::Continue),
                TestCases::FatalError => Err(WebSocketError::Fatal),
                _ => Ok(json! ({"ok":"200"})),
            }
        }
    }

    #[tokio::test]
    async fn test_listener_loop() {
        let mut receive = MockWebOsSocketTvReceive {
            case: TestCases::ContinueError,
        };

        let _error_contiue = InputPointerSocketConnection::listener_loop(&mut receive)
            .await
            .unwrap();

        receive.case = TestCases::FatalError;

        let fatal = InputPointerSocketConnection::listener_loop(&mut receive).await;

        assert!(fatal.is_err());

        if let Err(e) = fatal {
            let is_fatal = match e {
                WebSocketError::Continue => false,
                WebSocketError::Fatal => true,
            };

            assert!(is_fatal);
        }

        receive.case = TestCases::WrongJson;

        let _error_contiue = InputPointerSocketConnection::listener_loop(&mut receive)
            .await
            .unwrap();
    }

    struct MockWebOsTvRequestCommunication {
        case: TestCases,
    }

    #[async_trait]
    impl WebOsTvRequestCommunication for MockWebOsTvRequestCommunication {
        async fn send_json_request(
            &mut self,
            _json: Value,
        ) -> Result<PinkySwear<Value>, WebSocketError> {
            match self.case {
                TestCases::FatalError => Err(WebSocketError::Fatal),
                TestCases::ContinueError => Err(WebSocketError::Continue),
                TestCases::CorrectJson => {
                    let json_string = r#"{"id":0,"payload":{"returnValue":true,"socketPath":"ws://lgwebostv.local:3000/resources/eb0c0b658ad42ce05df0bf279a818017c5f11ae9/netinput.pointer.sock"},"type":"response"}"#;

                    let json: Value = serde_json::from_str(json_string).unwrap();
                    let promise = PinkySwear::new_with_data(json);

                    Ok(promise)
                }
                TestCases::WrongJson => {
                    let json_string = r#"{"id":0,"payload":{"returnValue":true,"socket2Path":"ws://lgwebostv.local:3000/resources/eb0c0b658ad42ce05df0bf279a818017c5f11ae9/netinput.pointer.sock"},"type":"response"}"#;

                    let json: Value = serde_json::from_str(json_string).unwrap();
                    let promise = PinkySwear::new_with_data(json);

                    Ok(promise)
                }
            }
        }
    }

    #[tokio::test]
    async fn test_get_socket_path() {
        let mut mock: Box<dyn WebOsTvRequestCommunication> =
            Box::new(MockWebOsTvRequestCommunication {
                case: TestCases::CorrectJson,
            });

        let address = InputPointerSocketConnection::get_socket_path(&mut mock)
            .await
            .unwrap();

        let expected_address =
        "ws://lgwebostv.local:3000/resources/eb0c0b658ad42ce05df0bf279a818017c5f11ae9/netinput.pointer.sock";

        assert_eq!(address, expected_address.to_string());

        let mut mock: Box<dyn WebOsTvRequestCommunication> =
            Box::new(MockWebOsTvRequestCommunication {
                case: TestCases::ContinueError,
            });

        let result = InputPointerSocketConnection::get_socket_path(&mut mock).await;

        assert_eq!(result.is_err(), true);

        let mut mock: Box<dyn WebOsTvRequestCommunication> =
            Box::new(MockWebOsTvRequestCommunication {
                case: TestCases::FatalError,
            });

        let result = InputPointerSocketConnection::get_socket_path(&mut mock).await;

        assert_eq!(result.is_err(), true);

        let mut mock: Box<dyn WebOsTvRequestCommunication> =
            Box::new(MockWebOsTvRequestCommunication {
                case: TestCases::WrongJson,
            });

        let result = InputPointerSocketConnection::get_socket_path(&mut mock).await;

        assert_eq!(result.is_err(), true);
    }
}
