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
    pub id: serde_json::Value,
    pub method: String,
    pub params: Option<serde_json::Value>,
}
impl Message for RequestMessage {}

// ResponseMessage extends Message
#[derive(Deserialize, Serialize, Debug)]
pub struct ResponseMessage {
    pub id: Option<serde_json::Value>,
    pub result: Option<serde_json::Value>,
    pub error: Option<serde_json::Value>, // Let's just use Value for Response Error
}
impl Message for ResponseMessage {}

#[derive(Deserialize, Serialize, Debug)]
pub struct InitializeResult {
    pub capabilities: serde_json::Value,
    pub serverInfo: Option<serde_json::Value>,
}
