//! # Web OS Client
//!
//! This Client has these follow features:
//! - Turn On TV, using the Wake On Lan protocol, se more in [`wake_on_lan`] module
//! - Discovery Tvs, Using the SSDP, se more in [`discovery`]
//! - Send Commands, like volume up,down, move to left,right,up,top
//! - Pointer Control, Move the mouse's position: dx,dy and the scroll
//! All commnads are in [`lg_command`]

pub mod client;
pub mod discovery;
pub mod lg_command;
pub mod wake_on_lan;
