use lg_webos_client::{
    client::{SendLgCommandRequest, WebOsClient, WebOsClientConfig, WebSocketErrorAction},
    lg_command::{CommandRequest, LGCommandRequest, REQUEST_TYPE},
};
use serde_json::json;

pub struct NewCommand;

impl LGCommandRequest for NewCommand {
    fn to_command_request(&self) -> CommandRequest {
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://com.webos.service.tv.power/turnOnScreen"),
            payload: Some(json! ({"standbyMode": "active"})),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), WebSocketErrorAction> {
    env_logger::init();
    // Note: We must specify the ws protocol, and if we do not have the key, we just specify None.
    let config = WebOsClientConfig {
        key: Some("2b61b97a6f97ce0325110c0a090f7874".to_string()),
        ..Default::default()
    };

    let mut client = WebOsClient::connect(config).await.unwrap();
    println!(
        "The key for next time you build WebOsClientConfig: {:?}",
        client.key.clone()
    );

    let json = client.send_lg_command_to_tv(NewCommand).await?;

    println!("{json:#}");
    Ok(())
}
