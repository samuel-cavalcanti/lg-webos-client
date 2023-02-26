use lg_webos_client::discovery::discovery_webostv_by_ssdp;

#[tokio::main]
async fn main() {
    let tv_info = discovery_webostv_by_ssdp().await;

    println!("Tv: {tv_info:?}");
}
