use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use log::debug;
use pinky_swear::Pinky;
use serde_json::Value;

pub struct WebOsSocketResponseListener {
    pending_requests: Arc<Mutex<HashMap<u8, Pinky<Value>>>>,
}

impl WebOsSocketResponseListener {
    pub fn new(
        pending_requests: Arc<Mutex<HashMap<u8, Pinky<Value>>>>,
    ) -> WebOsSocketResponseListener {
        WebOsSocketResponseListener { pending_requests }
    }

    pub fn receive_from_tv(&self, json: Value) {
        debug!("Receive JSON {}", &json);
        match json["id"].as_u64() {
            Some(res) => {
                let requests = self.pending_requests.lock().unwrap();
                let res = res as u8;

                match requests.get(&res) {
                    None => {
                        debug!("unable to find request of id {}", res);
                    }
                    Some(promise) => {
                        promise.swear(json);
                        debug!("resolved promise id: {}", res);
                    }
                };
            }
            None => {
                debug!("JSON response not have id");
            }
        };
    }
}
