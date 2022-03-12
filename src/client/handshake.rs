


use log::debug;
use serde_json::Value;



use crate::client::web_os_request_sender::WebOsTVRequestSender;


/// Get the initial handshake packet for connecting to a device.
/// A client-key can be set by something similar to
/// `get_handshake()["payload"]["client-key"] = ...`
/// # Return
/// The initial handshake packet needed to connect to a WebOS device.
pub async fn do_the_handshake(sender: &mut WebOsTVRequestSender,
) {
    debug!("sending handshake");
    /*
        when you not have any authentication key, the serve sends 2 responses.
        In the second response you may get the key if the connection is accepted
        otherwise key is None.
     */
    let handshake_request = get_handshake_request(sender.key.clone());


    let promise = sender.send_json(handshake_request).await;
    let key = parse_second_response_key(promise.await);
    sender.key = key;
}

fn parse_second_response_key(json: Value) -> Option<String> {
    let key = json
        .get("payload")
        .and_then(|p| p.get("client-key"))
        .and_then(|k| k.as_str())
        .map(Into::into);

    key
}

fn get_handshake_request(key: Option<String>) -> Value {
    let mut handshake = load_handshake_request_from_file();
    // Check to see if the config has a key, if it does, add it to the handshake.
    if let Some(key) = key {
        handshake["payload"]["client-key"] = Value::from(key);
        debug!("setting key to payload: {}", handshake["payload"]["client-key"]);
    }


    return handshake;
}

fn load_handshake_request_from_file() -> serde_json::Value {
    serde_json::from_str(include_str!("../handshake.json")).expect("Could not parse handshake json")
}