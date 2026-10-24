// use serde to serialize json into a struct, and viceversa
use serde::{Deserialize, Serialize};

// jsonrpc is always present and is always 2.0
fn jsonrpc() -> String {
    return String::from("2.0");
}

// RequestMessage extends Message
#[derive(Deserialize, Serialize, Debug)]
pub struct RequestMessage {
    #[serde(default = "jsonrpc")]
    pub jsonrpc: String,
    pub id: Option<serde_json::Value>,
    pub method: String,
    pub params: Option<serde_json::Value>,
}

// ResponseMessage extends Message
#[derive(Deserialize, Serialize, Debug)]
pub struct ResponseMessage {
    #[serde(default = "jsonrpc")]
    pub jsonrpc: String,
    pub id: Option<serde_json::Value>,
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>, // Let's just use Value for Response Error
}

#[derive(Deserialize, Serialize, Debug)]
pub struct InitializeResult {
    pub capabilities: serde_json::Value,
    pub serverInfo: Option<serde_json::Value>,
}
