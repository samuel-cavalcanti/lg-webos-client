use std::thread::sleep;
use std::time::Duration;

use log::{debug, error};
use serde_json::json;

use lg_webos_client::{
    client::{WebOsClient, WebOsClientConfig},
    lg_command::{pointer_input_commands::PointerInputCommand, LGCommandRequest},
};

use lg_webos_client::lg_command::{pointer_input_commands, request_commands};

#[tokio::main]
async fn main() {
    /*
        Testing some commands in my TV
    */
    env_logger::init();
    debug!("run on debug mode");

    // Note: We must specify the ws protocol, and if we do not have the key, we must use a blank str.
    let config = WebOsClientConfig::new(
        "ws://LGwebOSTV.local:3000/".to_string(),
        Some("2c9e3be310939e83013e4911637277c5".to_string()),
    );

    // let config = WebOsClientConfig::default();

    let client = WebOsClient::connect(config)
        .await
        .expect("Unable do connect");

    debug!("key: {:#?}", client.key.clone());

    let request_commands: Vec<Box<dyn LGCommandRequest>> = vec![
        Box::new(request_commands::system_notifications::CreateToast {
            message: "Hello TV!".to_string(),
        }),
        Box::new(request_commands::system_launcher::OpenBrowser {
            url: "https://www.google.com".to_string(),
        }),
        Box::new(request_commands::audio::SetMute { mute: true }),
        Box::new(request_commands::audio::AudioStatus),
        Box::new(request_commands::audio::SetMute { mute: false }),
        Box::new(request_commands::audio::SetVolume { volume: 15 }),
        Box::new(request_commands::system_launcher::LaunchApp {
            app_id: "netflix".to_string(),
            parameters: json!({}),
        }),
        Box::new(request_commands::web_os_services::GetPointerInputSocketUri),
    ];

    let input_commads: Vec<Box<dyn PointerInputCommand>> = vec![
        Box::new(pointer_input_commands::Button::HOME),
        Box::new(pointer_input_commands::Button::LEFT),
        Box::new(pointer_input_commands::Button::RIGHT),
        Box::new(pointer_input_commands::Button::UP),
        Box::new(pointer_input_commands::Button::DOWN),
        Box::new(pointer_input_commands::Button::BACK),
        Box::new(pointer_input_commands::Button::ENTER),
        Box::new(pointer_input_commands::Pointer::move_it(500.0, 0.0, false)),
        Box::new(pointer_input_commands::Pointer::move_it(
            500.0, 500.0, false,
        )),
        Box::new(pointer_input_commands::Pointer::scroll(0.0, 200.0)),
        Box::new(pointer_input_commands::Pointer::click()),
    ];

    for lg_command in request_commands {
        debug!("sending command to TV");
        match client.send_command_to_tv(lg_command).await {
            Ok(json) => debug!("Result: {:#?}", json),
            Err(_) => {
                error!("Error on send command");
            }
        }

        sleep(Duration::new(1, 0));
    }

    for command in input_commads {
        let result = client.send_pointer_input_command_to_tv(command).await;
        if let Err(e) = result {
            error!("Error on send input command {}", e);
        }
        sleep(Duration::new(2, 0));
    }
}
