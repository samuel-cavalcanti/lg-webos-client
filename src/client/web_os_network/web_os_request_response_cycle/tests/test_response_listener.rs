use crate::client::web_os_network::web_os_request_response_cycle::WebOsSocketResponseListener;
use pinky_swear::PinkySwear;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[tokio::test]
async fn test_response_listener() {
    let json_ids: Vec<u8> = vec![0, 1, 2];
    let ongoing_requests = Arc::new(Mutex::new(HashMap::new()));
    let listener = WebOsSocketResponseListener::new(ongoing_requests.clone());
    let mut promises: Vec<PinkySwear<Value>> = Vec::new();
    for id in json_ids.iter() {
        let (promise, pinky) = PinkySwear::<Value>::new();
        ongoing_requests.lock().unwrap().insert(id.clone(), pinky);
        promises.push(promise);
    }

    for (promise, json_id) in promises.iter().zip(json_ids) {
        let json = json!({"id":json_id,"test":json_id});
        listener.receive_from_tv(json.clone());
        let result = promise
            .try_wait()
            .expect("the promise should return an JSON");
        assert_eq!(result, json);
    }
}
