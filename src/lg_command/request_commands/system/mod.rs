//! # System commands Module
//! This module have commands that releated to ".service.tv" Uniform Resource   or "system" Uniform resource
//! locator.
//! Currently this module has only three commands: `TurnOffScreen`,`TurnOffTV`, `TurnOnScreen`
//!
//! ## Turn Of Tv Example:
//!
//!
//! ```
//! use lg_webos_client::client::SendLgCommandRequest;
//! use lg_webos_client::lg_command::LGCommandRequest;
//! use
//! lg_webos_client::lg_command::request_commands::system::TurnOffTV;
//!
//! async fn turn_of_tv<C:SendLgCommandRequest>(client:&mut C){
//!   
//!   let result = client.send_lg_command_to_tv(TurnOffTV).await;
//! }
//!
//! ```
//! If you want to troll someone, considere to use the Blink Example
//!
//! ## Blink example
//!
//! ```
//! use lg_webos_client::client::SendLgCommandRequest;
//! use lg_webos_client::lg_command::LGCommandRequest;
//! use std::time::Duration;
//! use lg_webos_client::lg_command::request_commands::system::{TurnOnScreen,TurnOffScreen};
//!
//! async fn blinking<C:SendLgCommandRequest>(client:&mut C){
//!   
//!   loop {
//!         let result = client.send_lg_command_to_tv(TurnOnScreen).await;
//!         std::thread::sleep(Duration::from_secs(1));
//!         let result = client.send_lg_command_to_tv(TurnOffScreen).await;
//!   }
//!   
//! }
//!
//! ```

mod turn_off_screen;
mod turn_off_tv;
mod turn_on_screen;

pub use turn_off_screen::TurnOffScreen;
pub use turn_off_tv::TurnOffTV;
pub use turn_on_screen::TurnOnScreen;
