use super::WebOsTvRequestCommunication;

pub struct Connection {
    pub key: String,
    pub request_sender: Box<dyn WebOsTvRequestCommunication>,
}
