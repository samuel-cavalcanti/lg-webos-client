use lg_webos_client::{
    client::{WebOsClient, WebOsClientConfig},
};


#[tokio::main]
async fn main() {
    env_logger::init();
    // Note: We must specify the ws protocol, and if we do not have the key, we just specify None.
    let config = WebOsClientConfig::default();
    let client = WebOsClient::connect(config).await.unwrap();
    println!(
        "The key for next time you build WebOsClientConfig: {:?}",
        client.key.clone()
    );
    let command = Box::new(lg_webos_client::lg_command::commands::tv::GetChannelList);
    let resp = client.send_command_to_tv(command).await;
    println!("Got response {:#?}", resp.expect("Error on send  get channel list"));
}
