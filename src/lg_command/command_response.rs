use serde_json::Value;

#[derive(Debug)]
pub struct CommandResponse {
    pub id: u8,
    pub payload: Option<Value>,
}
