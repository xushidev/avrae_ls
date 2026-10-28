// encode takes in a string (message), and returns the same string encoded in the format for LSP:
// Content-Length: <Bytes>\r\n\r\n<JSON_MESSAGE>
pub fn encode(msg: &str) -> String {
    let content = format!("Content-Length: {}\r\n\r\n{}", msg.as_bytes().len(), msg);
    return String::from(content);
}

