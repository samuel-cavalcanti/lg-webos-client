use lg_webos_client::{
    client::{WebOsClient, WebOsClientConfig},
    lg_command::request_commands::system_notifications::CreateToast,
};

use log::debug;
#[tokio::main]
async fn main() -> Result<(), String> {
    /*
        Testing some commands in my TV
    */
    env_logger::init();
    debug!("run on debug mode");

    // Testing if the client can be sended to another thread
    let join = tokio::task::spawn(async {
        // Note: We must specify the ws protocol, and if we do not have the key, we must use a blank str.
        let config = WebOsClientConfig::new(
            "ws://LGwebOSTV.local:3000/".to_string(),
            Some("2c9e3be310939e83013e4911637277c5".to_string()),
        );

        let mut client = WebOsClient::connect(config)
            .await
            .expect("Unable do connect");

        debug!("key: {:#?}", client.key.clone());

        let result = client
            .send_command_to_tv(Box::new(CreateToast {
                message: "HI".to_string(),
            }))
            .await;

        match result {
            Ok(output) => println!("\tout\n {}", output),
            Err(e) => println!("Error: {:?}", e),
        }
    });

    join.await.unwrap();
    Ok(())
}
