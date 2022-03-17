use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WebOsClientConfig {
    pub address: String,
    pub key: Option<String>,
}

impl ::std::default::Default for WebOsClientConfig {
    fn default() -> Self {
        WebOsClientConfig::new("ws://lgwebostv.local:3000/".to_string(), None)
    }
}

impl WebOsClientConfig {
    /// Creates a new client configuration
    pub fn new(address: String, key: Option<String>) -> WebOsClientConfig {
        WebOsClientConfig { address, key }
    }
}

impl Clone for WebOsClientConfig {
    fn clone(&self) -> Self {
        WebOsClientConfig {
            address: self.address.clone(),
            key: self.key.clone(),
        }
    }
}
