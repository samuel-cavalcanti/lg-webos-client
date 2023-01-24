pub trait PointerInputCommand {
    fn to_request_string(&self) -> String;
}

impl<T: PointerInputCommand + ?Sized> PointerInputCommand for Box<T> {
    fn to_request_string(&self) -> String {
        (**self).to_request_string()
    }
}
mod button;
mod pointer;
pub use button::Button;
pub use pointer::Pointer;
