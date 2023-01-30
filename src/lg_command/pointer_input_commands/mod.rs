pub trait PointerInputCommand: Send {
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
