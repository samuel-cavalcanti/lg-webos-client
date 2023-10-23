//! # WebOs Services Command Module
//!
//! This modules is related to "webos.service"  Uniform Resource.
//! Currently it's possible to get the WebOs mouse Uri, and maybe create some keyboard application
//! using `InsertText`, `DeleteCharacters``and `SendEnterKey`, But these three commands only work
//! if a virtual keyboard is showed
//!
//! Consider to Not use `GetPointerInputSocketUri`, use the `WebOsClient` with `SendPointerCommandRequest` instead
//!
//! maybe in the future will have a keyboard abstraction.
//!

mod get_current_services_information_list;
mod get_pointer_input_socket_uri;
mod set_display_3d;

mod delete_characters;
mod insert_text;
mod send_enter_key;

pub use get_current_services_information_list::GetCurrentServicesInformationList;
pub use get_pointer_input_socket_uri::GetPointerInputSocketUri;
pub use set_display_3d::SetDisplay3D;

pub use delete_characters::DeleteCharacters;
pub use insert_text::InsertText;
pub use send_enter_key::SendEnterKey;
