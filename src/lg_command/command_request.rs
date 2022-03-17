use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandRequest {
    pub r#type: String,
    pub uri: String,
    pub payload: Option<Value>,
}
