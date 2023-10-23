//! # Audio Commands modules
//! Commands that is related to audio like `VolumeUP`, `VolumeDown`
//!
//! ## Example
//! ```
//! use lg_webos_client::client::SendLgCommandRequest;
//! use lg_webos_client::lg_command::LGCommandRequest;
//! use
//! lg_webos_client::lg_command::request_commands::audio::{AudioStatus,GetVolume,SetMute,SetVolume,VolumeDown,VolumeUP};
//!
//! async fn example_to_use<C:SendLgCommandRequest>(client:&mut C){
//!   
//!   let result = client.send_lg_command_to_tv(AudioStatus).await;
//!   let result = client.send_lg_command_to_tv(GetVolume).await;
//!   let result = client.send_lg_command_to_tv(SetMute{mute:true}).await;
//!   let result = client.send_lg_command_to_tv(SetVolume{volume:10}).await;
//!   let result = client.send_lg_command_to_tv(VolumeUP).await;
//!   let result = client.send_lg_command_to_tv(VolumeDown).await;
//! }
//!
//! ```

mod audio_status;
mod get_volume;
mod set_mute;
mod set_volume;
mod volume_down;
mod volume_up;

pub use audio_status::AudioStatus;
pub use get_volume::GetVolume;
pub use set_mute::SetMute;
pub use set_volume::SetVolume;
pub use volume_down::VolumeDown;
pub use volume_up::VolumeUP;
