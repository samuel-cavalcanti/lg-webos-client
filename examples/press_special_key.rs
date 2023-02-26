use lg_webos_client::client::{SendPointerCommandRequest, WebOsClient, WebOsClientConfig};
use lg_webos_client::lg_command::pointer_input_commands::ButtonKey;

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

    let buttons = [
        ButtonKey::Num0,
        ButtonKey::Num1,
        ButtonKey::Num2,
        ButtonKey::Num3,
        ButtonKey::Num4,
        ButtonKey::Num5,
        ButtonKey::Num6,
        ButtonKey::Num7,
        ButtonKey::Num8,
        ButtonKey::Num9,
        //
        ButtonKey::GUIDE,
        ButtonKey::ENTER,
        ButtonKey::BACK,
        ButtonKey::DASH,
        ButtonKey::UP,
        ButtonKey::DOWN,
        ButtonKey::LEFT,
        ButtonKey::RIGHT,
    ];

    for b in buttons {
        let resp = client.send_pointer_input_command_to_tv(b).await;
        println!(
            "Got response {:#?}",
            resp.expect("Error on send  Button command")
        );
    }
}
