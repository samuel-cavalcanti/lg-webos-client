//!# Pointer Commnads
//!
//!The Basic Mouse Control, click, scroll, and move
//!
//!## Pointer Control Example
//!
//!```
//!use lg_webos_client::client::SendPointerCommandRequest;
//!use lg_webos_client::lg_command::pointer_input_commands::Pointer;
//!
//!const DRAG:bool = false;
//!const DS:f32 = 5.0;
//!async fn move_left<C:SendPointerCommandRequest>(client: &mut C){
//!   
//!   let _result = client.send_pointer_input_command_to_tv(Pointer::move_it(-DS, 0.0, DRAG)).await;
//!
//!}
//!async fn move_right<C:SendPointerCommandRequest>(client: &mut C){
//!   
//!   let _result = client.send_pointer_input_command_to_tv(Pointer::move_it(DS, 0.0, DRAG)).await;
//!
//!}
//!
//!async fn move_up<C:SendPointerCommandRequest>(client: &mut C){
//!   
//!   let _result = client.send_pointer_input_command_to_tv(Pointer::move_it(0.0, -DS, DRAG)).await;
//!
//!}
//!
//!async fn move_down<C:SendPointerCommandRequest>(client: &mut C){
//!   
//!   let _result = client.send_pointer_input_command_to_tv(Pointer::move_it(0.0, DS, DRAG)).await;
//!
//!}
//!
//!async fn click<C:SendPointerCommandRequest>(client: &mut C){
//!   
//!   let _result = client.send_pointer_input_command_to_tv(Pointer::click()).await;
//!
//!}
//!
//!
//!async fn scroll_down<C:SendPointerCommandRequest>(client: &mut C){
//!   
//!   let _result = client.send_pointer_input_command_to_tv(Pointer::scroll(0.0,DS)).await;
//!
//!}
//!
//!async fn scroll_up<C:SendPointerCommandRequest>(client: &mut C){
//!   
//!   let _result = client.send_pointer_input_command_to_tv(Pointer::scroll(0.0,-DS)).await;
//!
//!}
//!```

use super::PointerInputCommand;

/// The commands for Move,click and scroll the Webos's mouses
pub struct Pointer {
    command: String,
}

impl Pointer {
    pub fn move_it(dx: f32, dy: f32, drag: bool) -> Pointer {
        let drag = match drag {
            true => "1",
            false => "0",
        };

        Pointer {
            command: format!("type:move\ndx:{dx}\ndy:{dy}\ndown:{drag}\n\n"),
        }
    }

    pub fn scroll(dx: f32, dy: f32) -> Pointer {
        Pointer {
            command: format!("type:scroll\ndx:{dx}\ndy:{dy}\n\n"),
        }
    }

    pub fn click() -> Pointer {
        Pointer {
            command: "type:click\n\n".to_string(),
        }
    }
}

impl PointerInputCommand for Pointer {
    fn to_request_string(&self) -> String {
        self.command.clone()
    }
}
