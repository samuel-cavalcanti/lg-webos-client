//! # System Launcher Commands
//!
//! This module is related to Launch applincations on your TV, like open Netflix or YouTube.
//!
//!
//! ## Open Netflix Example:
//!
//! ```
//! use lg_webos_client::client::SendLgCommandRequest;
//! use lg_webos_client::lg_command::LGCommandRequest;
//! use lg_webos_client::lg_command::request_commands::system_launcher::LaunchApp;
//!
//! async fn open_netflix<C:SendLgCommandRequest>(client:&mut C){
//!   
//!   let result = client.send_lg_command_to_tv(LaunchApp::netflix()).await;
//!   // LaunchApp also supports, Amazon Prime and YouTube.
//!   let _amazon = LaunchApp::amazon_prime_video();
//!   let _youtube = LaunchApp::youtube();
//! }
//! ```
//!
//! That also possible to Open the WebOs Web browser
//!
//! ## Web browser Example
//!
//! ```
//! use lg_webos_client::lg_command::request_commands::system_launcher::OpenBrowser;
//! use lg_webos_client::client::SendLgCommandRequest;
//! use lg_webos_client::lg_command::LGCommandRequest;
//! async fn open_browser<C:SendLgCommandRequest>(client:&mut C){
//!
//!   let result = client.send_lg_command_to_tv(OpenBrowser{ url: "https://www.google.com/".into()}).await;
//! }
//! ```

mod launch_app;
mod list_apps;
mod open_browser;
pub use launch_app::LaunchApp;
pub use list_apps::ListApps;
pub use open_browser::OpenBrowser;
