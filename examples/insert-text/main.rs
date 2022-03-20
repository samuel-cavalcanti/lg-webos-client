use lg_webos_client::client::{WebOsClient, WebOsClientConfig};
use lg_webos_client::lg_command::request_commands;

#[tokio::main]
async fn main() {
    env_logger::init();
    /*
        In this exemple I open the web browser in my LG TV and  put on google.com
        when appears the virtual keyboard To search, I run this exemple.
    */
    let config = WebOsClientConfig::new(
        "ws://lgwebostv.local:3000/".to_string(),
        Some("911232e4ea63476203fde58db0a0e793".to_string()),// my Key
    );
    let client = WebOsClient::connect(config).await.unwrap();
    println!(
        "The key for next time you build WebOsClientConfig: {:?}",
        client.key.clone()
    );
    let command = Box::new(request_commands::web_os_services::InsertText {
        text: "test".to_string(),
        replace: false,
    });
    let resp = client.send_command_to_tv(command).await;
    println!(
        "Got response {:#?}",
        resp.expect("Error on send  send text")
    );
}
