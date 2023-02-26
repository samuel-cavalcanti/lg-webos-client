use lg_webos_client::client::{SendLgCommandRequest, WebOsClient, WebOsClientConfig};
use lg_webos_client::lg_command::request_commands::system_launcher::{self, ListApps};

#[derive(Debug)]
struct AppInfo {
    id: String,
    name: String,
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
    let json = client.send_lg_command_to_tv(ListApps).await.unwrap();

    let all_apps = json["payload"]["apps"]
        .as_array()
        .unwrap()
        .into_iter()
        .map(|app_json| AppInfo {
            id: app_json["id"].as_str().unwrap().to_string(),
            name: app_json["title"].as_str().unwrap().to_string(),
        });

    for app_info in all_apps {
        println!("{app_info:#?}");
    }
    let apps = [
        system_launcher::LaunchApp::youtube(),
        system_launcher::LaunchApp::amazon_prime_video(),
        system_launcher::LaunchApp::netflix(),
    ];

    for app in apps {
        let resp = client.send_lg_command_to_tv(app).await;
        println!(
            "Got response {:#?}",
            resp.expect("Error on send  launch app")
        );
    }
}
