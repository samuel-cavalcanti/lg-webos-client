use async_trait::async_trait;

use crate::client::web_os_network::handshake::handshake::HandShake;
use crate::client::web_os_network::{WebOsSocketTvReceive, WebOsSocketTvSend};
use crate::client::WebSocketErrorAction;
use serde_json::Value;

struct MockNewClientReceive {
    number_of_responses: u8,
    only_one_response: bool,
    client_gonna_refuse: bool,
}

impl MockNewClientReceive {
    fn get_json(&self, number_of_responses: u8) -> Result<Value, WebSocketErrorAction> {
        let first_json =
            r#"{"id":0,"payload":{"pairingType":"PROMPT","returnValue":true},"type":"response"}"#;
        let register_json = r#"{"id":0,"payload":{"client-key":"6f276548085335decbfd5f5a00689c71"},"type":"registered"}"#;
        let error_json =
            r#"{"error":"403 Error: User rejected pairing","id":0,"payload":{},"type":"error"}"#;

        if number_of_responses == 0 {
            if self.only_one_response {
                Ok(serde_json::from_str::<Value>(&register_json).unwrap())
            } else {
                Ok(serde_json::from_str::<Value>(&first_json).unwrap())
            }
        } else if number_of_responses == 1 {
            if self.only_one_response {
                Err(WebSocketErrorAction::Fatal)
            } else {
                if self.client_gonna_refuse {
                    Ok(serde_json::from_str::<Value>(&error_json).unwrap())
                } else {
                    Ok(serde_json::from_str::<Value>(&register_json).unwrap())
                }
            }
        } else {
            Err(WebSocketErrorAction::Fatal)
        }
    }
}

#[async_trait]
impl WebOsSocketTvReceive for MockNewClientReceive {
    async fn receive(&mut self) -> Result<Value, WebSocketErrorAction> {
        let json = self.get_json(self.number_of_responses);

        self.number_of_responses += 1;

        return json;
    }
}

struct MockSend;

#[async_trait]
impl WebOsSocketTvSend for MockSend {
    async fn send(&mut self, _: Value) -> Result<(), WebSocketErrorAction> {
        Ok(())
    }
}

#[tokio::test]
async fn test_success_new_client() {
    let mut receiver = MockNewClientReceive {
        number_of_responses: 0,
        only_one_response: false,
        client_gonna_refuse: false,
    };
    let mut sender = MockSend;

    let key = HandShake::do_the_handshake(&mut sender, &mut receiver, None).await;

    assert_eq!(
        key.clone().unwrap(),
        "6f276548085335decbfd5f5a00689c71".to_string()
    );
    /*
        when the client send the handshake with no key. The Tv gona send 2 responses with on the second
        gona have an key, so the HandShake::do_the_handshake must wait 2 responses.
    */
    assert_eq!(receiver.number_of_responses, 2);

    /*
        when the client send the handshake with an correct key, the TV gone to send only 1 response
    */
    receiver = MockNewClientReceive {
        number_of_responses: 0,
        only_one_response: true,
        client_gonna_refuse: false,
    };
    let key = HandShake::do_the_handshake(&mut sender, &mut receiver, Some(key.unwrap())).await;

    assert_eq!(
        key.clone().unwrap(),
        "6f276548085335decbfd5f5a00689c71".to_string()
    );
    assert_eq!(receiver.number_of_responses, 1);
}

#[tokio::test]
async fn test_fail_new_client() {
    let mut receiver = MockNewClientReceive {
        number_of_responses: 0,
        only_one_response: false,
        client_gonna_refuse: true,
    };
    let mut sender = MockSend;

    /*
        When the Client not have a key and the user of TV refuse to accept the connection, so The Tv
        gonna send an error on the second response.
    */
    let key = HandShake::do_the_handshake(&mut sender, &mut receiver, None).await;
    assert_eq!(receiver.number_of_responses, 2);
    assert_eq!(key.is_err(), true);
}
