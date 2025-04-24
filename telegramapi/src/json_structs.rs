use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SendMessageResponse {
    pub ok: bool,
    pub result: Option<Message>,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub message_id: i64,
    pub text: Option<String>,
}