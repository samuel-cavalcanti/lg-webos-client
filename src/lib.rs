//! # Web OS Client
//!
//! This Client has these follow features:
//! - Turn On TV, using the Wake On Lan protocol, se more in [`wake_on_lan`] module
//! - Discovery Tvs, Using the SSDP, se more in [`discovery`]
//! - Send Commands, like volume up,down, move to left,right,up,top
//! - Pointer Control, Move the mouse's position: dx,dy and the scroll.  
//! All commnads are in [`lg_command`]
//!
//!
//! ## Get Stated
//!
//!This example will create a Toast 
//!
//!```no_run
//!
//!use lg_webos_client::{
//!   client::{SendLgCommandRequest, WebOsClient, WebOsClientConfig},
//!   lg_command::request_commands::system_notifications::CreateToast,
//!};
//!
//! #[tokio::main]
//! async fn main(){
//!
//!     let config = WebOsClientConfig {
//!         // When you don't have a key, will apear a message
//!         // on screen, select Ok.
//!         key: None,
//!         // If your Os Have mDns you can use this,
//!         // otherwise, you must set the correct ip
//!         // example: ws://{ip}:3000/
//!         address: "ws://lgwebostv.local:3000/".into(),
//!      };
//!
//!     let mut client = WebOsClient::connect(config)
//!         .await
//!         .expect("Unable do connect");
//!
//!     client.send_lg_command_to_tv(CreateToast {
//!             message: "Hello World".to_string(),
//!         })
//!         .await.expect("Unable to create a Toast");
//!
//! }
//! 
//!```

pub mod client;
pub mod discovery;
pub mod lg_command;
pub mod wake_on_lan;
