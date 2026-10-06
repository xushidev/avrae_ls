// use serde to serialize json into a struct, and viceversa
use serde::{Deserialize, Serialize};

// jsonrpc is always present and is always 2.0
trait Message {
    fn jsonrpc() -> String {
        return String::from("2.0");
    }
}

// RequestMessage extends Message
#[derive(Deserialize, Serialize, Debug)]
pub struct RequestMessage {
    id: serde_json::Value,
    method: String,
    params: Option<serde_json::Value>,
}
impl Message for RequestMessage {}

// ResponseMessage extends Message
#[derive(Deserialize, Serialize, Debug)]
struct ResponseMessage {
    id: Option<serde_json::Value>,
    result: Option<serde_json::Value>,
    error: Option<serde_json::Value>, // Let's just use Value for Response Error
}
impl Message for ResponseMessage {}

// base message has method parameter
#[derive(Deserialize, Serialize)]
pub struct BaseMessage {
    pub method: String,
}
