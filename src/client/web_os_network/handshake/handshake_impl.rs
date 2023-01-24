use crate::client::web_os_network::{WebOsSocketTvReceive, WebOsSocketTvSend};
use log::debug;
use serde_json::Value;

pub struct HandShake;

const ERROR_MESSAGE: &str = "Unable do make HandShake";

impl HandShake {
    /// Get the initial handshake packet for connecting to a device.
    /// A client-key can be set by something similar to
    /// `get_handshake()["payload"]["client-key"] = ...`
    /// # Return
    /// The initial handshake packet needed to connect to a WebOS device.
    pub async fn do_the_handshake<S: WebOsSocketTvSend, R: WebOsSocketTvReceive>(
        sender: &mut S,
        receiver: &mut R,
        key: Option<String>,
    ) -> Result<String, String> {
        debug!("sending handshake");
        /*
           when you not have any authentication key, the serve sends 2 responses.
           In the second response you may get the key if the connection is accepted
           otherwise key is None.
        */
        let handshake_request = HandShake::get_handshake_request(key);
        let handshake_request = serde_json::to_string(&handshake_request)
            .expect("Unable to Convert handshake_request to String");
        if sender.send_text(handshake_request).await.is_err() {
            return Err(ERROR_MESSAGE.to_string());
        }

        let first_package = HandShake::try_to_receive(receiver).await?;
        debug!("First JSON {}", first_package);
        let is_register_response = HandShake::is_register_response(first_package["type"].as_str())?;

        if is_register_response {
            return HandShake::try_to_parse_response_key(first_package);
        }

        let second_package = HandShake::try_to_receive(receiver).await?;
        debug!("Second JSON {}", second_package);

        HandShake::try_to_parse_response_key(second_package)
    }

    fn is_register_response(json_type: Option<&str>) -> Result<bool, String> {
        match json_type {
            Some(response_type) => {
                if response_type == "registered" || response_type == "error" {
                    /*
                       when the user not authorize the app,
                       The Tv register response returns "type":"error"
                    */
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            None => Err("Unable to Parse id".to_string()),
        }
    }

    async fn try_to_receive<R: WebOsSocketTvReceive>(receiver: &mut R) -> Result<Value, String> {
        match receiver.receive().await {
            Ok(json) => Ok(json),
            Err(_) => Err(ERROR_MESSAGE.to_string()),
        }
    }

    fn try_to_parse_response_key(json: Value) -> Result<String, String> {
        let key = json
            .get("payload")
            .and_then(|p| p.get("client-key"))
            .and_then(|k| k.as_str())
            .map(Into::into);

        match key {
            Some(key) => Ok(key),
            None => Err(ERROR_MESSAGE.to_string()),
        }
    }

    fn get_handshake_request(key: Option<String>) -> Value {
        let mut handshake = HandShake::load_handshake_request_from_file();
        // Check to see if the config has a key, if it does, add it to the handshake.
        if let Some(key) = key {
            handshake["payload"]["client-key"] = Value::from(key);
            debug!(
                "setting key to payload: {}",
                handshake["payload"]["client-key"]
            );
        }

        handshake
    }

    fn load_handshake_request_from_file() -> serde_json::Value {
        serde_json::from_str(include_str!("./handshake.json"))
            .expect("Could not parse handshake json")
    }
}
