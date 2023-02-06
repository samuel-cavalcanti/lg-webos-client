use lg_webos_client::client::{SendLgCommandRequest, WebOsClient, WebOsClientConfig};
use lg_webos_client::lg_command::request_commands::system_launcher;
use lg_webos_client::lg_command::{CommandRequest, LGCommandRequest, REQUEST_TYPE};

//ssap://com.webos.applicationManager/listApps
struct ListApps;

impl LGCommandRequest for ListApps {
    fn to_command_request(&self) -> CommandRequest {
        CommandRequest {
            r#type: REQUEST_TYPE.to_string(),
            uri: String::from("ssap://com.webos.applicationManager/listApps"),
            payload: None,
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    // Note: We must specify the ws protocol, and if we do not have the key, we just specify None.
    let config = WebOsClientConfig {
        key: Some("cb21f4ad19a2438a67671d874584b875".to_string()),
        ..Default::default()
    };

    let mut client = WebOsClient::connect(config).await.unwrap();
    println!(
        "The key for next time you build WebOsClientConfig: {:?}",
        client.key.clone()
    );

    let apps = [
        system_launcher::LaunchApp::youtube(),
        system_launcher::LaunchApp::amazon_prime_video(),
        system_launcher::LaunchApp::netflix(),
    ];

    for app in apps {
        let resp = client.send_lg_command_to_tv(app).await;
        println!(
            "Got response {:#?}",
            resp.expect("Error on send  get channel list")
        );
    }
}
