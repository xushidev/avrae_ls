// import the required structs
use crate::structs::*;
// import the logging method
use log::info;

// encode takes in a string (message), and returns the same string encoded in the format for LSP:
// Content-Length: <Bytes>\r\n\r\n<JSON_MESSAGE>
pub fn encode(msg: &str) -> String {
    let content = format!("Content-Length: {}\r\n\r\n{}", msg.as_bytes().len(), msg);
    // Logging
    info!("Encoded content: {}", content);
    return String::from(content);
}

// decode takes in a string (message), and returns it's contents;
// panics at an eventual error in decoding
pub fn decode(msg: &str) -> RequestMessage {
    // splits the message between it's header and the content before unwrapping them
    let mut msg_parts = msg.split("\r\n\r\n");
    let (header, content) = (msg_parts.next().unwrap(), msg_parts.next().unwrap());

    // get the content length from the header and
    // return the content size to read
    let content_length: usize = header["Content-Length: ".len()..].parse().unwrap();

    // turn the string into an RequestMessage object
    let request_message: RequestMessage = serde_json::from_str(&content[..content_length]).unwrap();

    return request_message;
}
