use super::WebOsTvRequestCommunication;

pub struct Connection {
    pub key: String,
    pub sender: Box<dyn WebOsTvRequestCommunication>,
}
