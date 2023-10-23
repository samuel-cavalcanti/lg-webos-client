//! # Media Controls
//!
//! Commands that controls to video flow, like: `Pause`,`Play`,`Stop`.
//!
//! ## Example
//!
//! ```
//! use lg_webos_client::client::SendLgCommandRequest;
//! use lg_webos_client::lg_command::LGCommandRequest;
//! use
//! lg_webos_client::lg_command::request_commands::media_controls::{Play,Stop,Pause,Rewind,FastForward};
//!
//! async fn example_to_use<C:SendLgCommandRequest>(client:&mut C){
//!   
//!   let result = client.send_lg_command_to_tv(Play).await;
//!   let result = client.send_lg_command_to_tv(Stop).await;
//!   let result = client.send_lg_command_to_tv(Pause).await;
//!   let result = client.send_lg_command_to_tv(Rewind).await;
//!   let result = client.send_lg_command_to_tv(FastForward).await;
//! }
//! ```
//!
//!

mod fast_forward;
mod pause;
mod play;
mod rewind;
mod stop;

pub use fast_forward::FastForward;
pub use pause::Pause;
pub use play::Play;
pub use rewind::Rewind;
pub use stop::Stop;
