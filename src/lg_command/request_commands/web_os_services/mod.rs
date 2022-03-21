mod get_current_services_information_list;
mod get_pointer_input_socket_uri;
mod set_display_3d;

/*
    insert_text, send_enter_key, delete_characters
    works only if a virtual keyboard is on screen.
*/
mod delete_characters;
mod insert_text;
mod send_enter_key;

pub use get_current_services_information_list::GetCurrentServicesInformationList;
pub use get_pointer_input_socket_uri::GetPointerInputSocketUri;
pub use set_display_3d::SetDisplay3D;

pub use delete_characters::DeleteCharacters;
pub use insert_text::InsertText;
pub use send_enter_key::SendEnterKey;
