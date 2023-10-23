//! # TV Commannds module
//!
//! This Module is releted with "/tv/" Uniform Resource, some of Commannds are:
//! - `ChannelUp`: increase the channel number to 1
//! - `ChannelDown`: decrease the channel number to 1
//! - `GetExternalInputList` Enter in the GetExternalInput screen
//!
//! ## Change the current Channel example:
//!
//! supose that the current channel is 10, if you use `increase_channel`
//! then  the current channel will be 11
//! if you use  decrease_channel, the current channel will be 9.
//! Every Open Channel has a id, so you can Get the Channel Id using `GetChannelList`,
//! `GetOpenChannelInformation` or
//! `GetCurrentChannelInformation`, and use `set_channel` function
//!
//! ```
//! use lg_webos_client::client::SendLgCommandRequest;
//! use lg_webos_client::lg_command::LGCommandRequest;
//! use lg_webos_client::lg_command::request_commands::tv::{ChannelDown,ChannelUp,SetOpenChannel};
//!
//! async fn increase_channel<C:SendLgCommandRequest>(client:&mut C){
//!   
//!   let result = client.send_lg_command_to_tv(ChannelUp).await;
//! }
//! async fn decrease_channel<C:SendLgCommandRequest>(client:&mut C){
//!   
//!   let result = client.send_lg_command_to_tv(ChannelDown).await;
//! }
//!
//! async fn set_channel<C:SendLgCommandRequest>(client:&mut C, channel_id:String){
//!
//!   let result = client.send_lg_command_to_tv(SetOpenChannel{channel_id}).await;
//!
//! }
//! ```
//!
//!

mod channel_down;
mod channel_up;
mod get_channel_list;
mod get_current_channel_information;
mod get_open_channel_information;
mod set_open_channel;

mod get_external_input_list;
mod switch_input;

pub use channel_down::ChannelDown;
pub use channel_up::ChannelUp;
pub use get_channel_list::GetChannelList;
pub use get_current_channel_information::GetCurrentChannelInformation;
pub use get_open_channel_information::GetOpenChannelInformation;
pub use set_open_channel::SetOpenChannel;

pub use get_external_input_list::GetExternalInputList;
pub use switch_input::SwitchInput;
