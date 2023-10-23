//! # Request Commnads Module
//!
//! All Comands that can sended with [`SendLgCommandRequest`].
//! Current this module is subdivided in:
//! - [`audio`]
//! - [`system`]
//! - [`system_launcher`]
//! - [`tv`]
//! - [`web_os_services`]
//!
//! This division is based in the Uniform Resource Identifier (URI)

#[cfg(doc)]
use crate::client::SendLgCommandRequest;

use crate::lg_command::{CommandRequest, LGCommandRequest};

pub mod media_controls;

pub mod audio;
pub mod system;
pub mod system_launcher;
pub mod system_notifications;
pub mod tv;
pub mod web_os_services;
