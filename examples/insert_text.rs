use std::thread::sleep;
use std::time::Duration;

use lg_webos_client::client::{SendLgCommandRequest, WebOsClient, WebOsClientConfig};
use lg_webos_client::lg_command::{request_commands, LGCommandRequest};

#[tokio::main]
async fn main() {
    env_logger::init();
    /*
        In this exemple I open the web browser in my LG TV and  put on google.com
        when appears the virtual keyboard To search, I run this exemple.
    */
    let config = WebOsClientConfig::new(
        "ws://lgwebostv.local:3000/".to_string(),
        Some("023795a2c038fbce7ff4a97d8b362441".to_string()), // my Key
    );
    let mut client = WebOsClient::connect(config).await.unwrap();
    println!(
        "The key for next time you build WebOsClientConfig: {:?}",
        client.key.clone()
    );
    let text = String::from("test");

    let request_commands: Vec<Box<dyn LGCommandRequest>> = vec![
        Box::new(request_commands::web_os_services::InsertText {
            text: text.clone(),
            replace: false,
        }),
        Box::new(request_commands::web_os_services::InsertText {
            text: text.clone(),
            replace: false,
        }),
        Box::new(request_commands::web_os_services::DeleteCharacters {
            number_of_chars: text.len(),
        }),
        Box::new(request_commands::web_os_services::InsertText {
            text: String::from(" Success"),
            replace: false,
        }),
        Box::new(request_commands::web_os_services::SendEnterKey),
    ];

    for command in request_commands {
        let resp = client.send_lg_command_to_tv(command).await;
        println!("Got response {:#?}", resp.expect("Error on send request"));
        sleep(Duration::new(2, 0));
    }
}
