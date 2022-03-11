use lg_webos_client::{
    client::{web_os_client::WebosClient, web_os_client_config::WebOsClientConfig},
};


#[tokio::main]
async fn main() {
    env_logger::init();
    // Note: We must specify the ws protocol, and if we do not have the key, we just specify None.
    let config = WebOsClientConfig::default();
    let client = WebosClient::new(config).await.unwrap();
    println!(
        "The key for next time you build WebOsClientConfig: {:?}",
        client.key
    );
    let command = Box::new(lg_webos_client::lg_command::commands::tv::GetChannelList);
    let resp = client.send_command(command).await.unwrap();
    println!("Got response {:?}", resp.payload);
}
