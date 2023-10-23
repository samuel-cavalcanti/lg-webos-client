//! # Notifications Commands Module
//!
//! You can also create a Toast to WebOs TV with some Message
//!
//! ## Toast Example
//!
//! ```
//! use lg_webos_client::client::SendLgCommandRequest;
//! use lg_webos_client::lg_command::LGCommandRequest;
//! use lg_webos_client::lg_command::request_commands::system_notifications::CreateToast;
//!
//! async fn hello_world<C:SendLgCommandRequest>(client:&mut C){
//!   
//!   let toast = CreateToast { message: "Hello World!!".into() };
//!   let result = client.send_lg_command_to_tv(toast).await;
//! }
//! ```

pub mod create_toast;

pub use create_toast::CreateToast;
