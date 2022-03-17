use lg_webos_client::lg_command::CommandRequest;

mod audio;
mod media_controls;
mod system;
mod system_launcher;
mod system_notifications;
mod tv;
mod web_os_services;

fn assert_command_request(result: CommandRequest, expected: CommandRequest) {
    assert_eq!(result.uri, expected.uri);
    assert_eq!(result.r#type, expected.r#type);
    assert_eq!(result.payload, expected.payload);
}
