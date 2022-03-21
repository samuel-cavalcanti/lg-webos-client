use crate::client::web_os_network::{
    WebOsSocketTvSend, WebOsTVRequestSender, WebOsTvRequestCommunication,
};
use crate::client::WebSocketErrorAction;
use async_trait::async_trait;
use serde_json::json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct MockTVSend;

#[async_trait]
impl WebOsSocketTvSend for MockTVSend {
    async fn send_text(&mut self, _: String) -> Result<(), WebSocketErrorAction> {
        Ok(())
    }
}

#[tokio::test]
async fn test_request_sender() {
    let id = Arc::new(Mutex::new(0));
    let ongoing_requests = Arc::new(Mutex::new(HashMap::new()));
    let mut sender = WebOsTVRequestSender::new(Box::new(MockTVSend), id, ongoing_requests.clone());

    for (wrong_id, correct_id) in vec![32, 22, 12].iter().zip(vec![0, 1, 2]) {
        let test_json = json!({"id":wrong_id,"test":"123"});
        let result = sender.send_json_request(test_json).await;
        assert_eq!(result.is_ok(), true);
        assert_eq!(
            ongoing_requests.lock().unwrap().get(&correct_id).is_some(),
            true
        );
    }
}
