//!# Pointer Inputs Commands Module
//!
//!Pointer Inputs has commnads like: **home**,**back**,**Enter** and can control the **position** of
//!Webos's Mouse and the **scroll**. If you want to control the mouse see [`Pointer`] commnads,
//!if you and to know about the mouse's special buttons see [`ButtonKey`].
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
//!   let left =Pointer::move_it(-DS, 0.0, DRAG);
//!   
//!   let _result = client.send_pointer_input_command_to_tv(left).await;
//!
//!}
//!async fn move_right<C:SendPointerCommandRequest>(client: &mut C){
//!
//!    let right = Pointer::move_it(DS, 0.0, DRAG);
//!   
//!   let _result = client.send_pointer_input_command_to_tv(right).await;
//!
//!}
//!
//!async fn move_up<C:SendPointerCommandRequest>(client: &mut C){
//!   
//!   let up = Pointer::move_it(0.0, -DS, DRAG);
//!   let _result = client.send_pointer_input_command_to_tv(up).await;
//!
//!}
//!
//!async fn move_down<C:SendPointerCommandRequest>(client: &mut C){
//!   
//!   let down = Pointer::move_it(0.0, DS, DRAG);
//!   let _result = client.send_pointer_input_command_to_tv(down).await;
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
//!
//!## Mouse Especial Keys Example
//!
//!```
//!use lg_webos_client::client::SendPointerCommandRequest;
//!use lg_webos_client::lg_command::pointer_input_commands::ButtonKey;
//!
//!async fn press_home<C:SendPointerCommandRequest>(client: &mut C){
//!   
//!   let _result = client.send_pointer_input_command_to_tv(ButtonKey::HOME).await;
//!
//!}
//!
//!async fn press_back<C:SendPointerCommandRequest>(client: &mut C){
//!   
//!   let _result = client.send_pointer_input_command_to_tv(ButtonKey::BACK).await;
//!
//!}
//!
//!async fn press_left<C:SendPointerCommandRequest>(client: &mut C){
//!   
//!   let _result = client.send_pointer_input_command_to_tv(ButtonKey::LEFT).await;
//!
//!}
//!
//!async fn press_right<C:SendPointerCommandRequest>(client: &mut C){
//!   
//!   let _result = client.send_pointer_input_command_to_tv(ButtonKey::RIGHT).await;
//!
//!}
//!
//!async fn press_up<C:SendPointerCommandRequest>(client: &mut C){
//!   
//!   let _result = client.send_pointer_input_command_to_tv(ButtonKey::UP).await;
//!
//!}
//!
//!async fn press_down<C:SendPointerCommandRequest>(client: &mut C){
//!   
//!   let _result = client.send_pointer_input_command_to_tv(ButtonKey::DOWN).await;
//!
//!}
//!```

/// This trait Represents WebOS Pointer Request
pub trait PointerInputCommand: Send {
    /// Basicly the TV expects a message with this follow format:  
    /// **\<id>:\<value>\n**  
    /// When  the lest \<id>:\<value> must have 2 end lines:  
    /// **\<id>:\<value>\n\n**  
    fn to_request_string(&self) -> String;
}

impl<T: PointerInputCommand + ?Sized> PointerInputCommand for Box<T> {
    fn to_request_string(&self) -> String {
        (**self).to_request_string()
    }
}
mod button_key;
mod pointer;
pub use button_key::ButtonKey;

pub use pointer::Pointer;
