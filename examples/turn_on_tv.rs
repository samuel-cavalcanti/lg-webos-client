use std::{thread, time::Duration};

use lg_webos_client::{
    client::{SendLgCommandRequest, WebOsClient, WebOsClientConfig},
    discovery,
    lg_command::request_commands::system::TurnOffTV,
    wake_on_lan,
};

#[tokio::main]
async fn main() {
    // Execute this example with the TV on
    let tv_info = discovery::discovery_webostv_by_ssdp().await.unwrap();

    let tv_info = &tv_info[0];

    let tv_ip = &tv_info.ip;
    let config = WebOsClientConfig {
        address: format!("ws://{tv_ip}:3000/"),
        key: Some("2b61b97a6f97ce0325110c0a090f7874".to_string()),
    };
    // Look to your TV and  Accept the connection
    let mut client = WebOsClient::connect(config.clone()).await.unwrap();

    thread::sleep(Duration::from_secs(1));

    client.send_lg_command_to_tv(TurnOffTV).await.unwrap();

    thread::sleep(Duration::from_secs(1));

    let magic_package = wake_on_lan::MagicPacket::from_mac_string(&tv_info.mac_address).unwrap();
    wake_on_lan::send_magic_packet_to_address(magic_package, &format!("{tv_ip}:9"))
        .await
        .unwrap();

    let mut tv_info = discovery::discovery_webostv_by_ssdp().await.unwrap();
    while tv_info.is_empty() {
        tv_info = discovery::discovery_webostv_by_ssdp().await.unwrap();
    }
    client = WebOsClient::connect(config).await.unwrap();

    thread::sleep(Duration::from_secs(1));

    client.send_lg_command_to_tv(TurnOffTV).await.unwrap();
}
