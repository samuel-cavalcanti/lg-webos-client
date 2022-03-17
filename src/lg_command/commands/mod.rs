use crate::lg_command::{CommandRequest, LGCommandRequest};

pub mod media_controls;

pub mod audio;
pub mod system;
pub mod system_launcher;
pub mod system_notifications;
pub mod tv;
pub mod web_os_services;

/*
   Pointer Input Socket seems to be the TV remote Controller
   like, Home,Back,Enter button.
*/
pub mod pointer_input_socket;
