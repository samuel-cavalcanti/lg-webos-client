use serde_json::Value;

#[derive(Debug)]
/// The Response Expeceted form Request, but currently isn't used  
pub struct CommandResponse {
    pub id: u8,
    pub payload: Option<Value>,
}
