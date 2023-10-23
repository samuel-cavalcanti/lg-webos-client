/// Parameters To connect to TV
/// if the Client isn't authenticated then the key must be None
/// The address must contain the port,and the connection type. The Default **port is 3000**
/// The default connection is Websocket: **ws://**
/// If you Operational System has mDns the ip can be: **lgwebostv.local**
/// ```
/// use lg_webos_client::client::WebOsClientConfig;
///
/// let config = WebOsClientConfig {
///     address: "ws://192.1680.0.2:3000/".into(),
///     key: None,
/// };
/// ```
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
