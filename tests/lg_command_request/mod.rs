use lg_webos_client::lg_command::{CommandRequest};

mod media_controls;
mod system_notifications;
mod tv;
mod audio;
mod system_launcher;
mod web_os_services;
mod system;


fn assert_command_request(result: CommandRequest, expected: CommandRequest) {
    assert_eq!(result.uri, expected.uri);
    assert_eq!(result.r#type, expected.r#type);
    assert_eq!(result.payload, expected.payload);
}
