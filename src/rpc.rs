// import the required structs
use crate::structs::*;
// import the logging method
use log::info;

// encode takes in a string (message), and returns the same string encoded in the format for LSP:
// Content-Length: <Bytes>\r\n\r\n<JSON_MESSAGE>
pub fn encode(msg: &str) -> String {
    let content = format!(
        "Content-Length: {}\r\n\r\n{}",
        msg.as_bytes().len() + 4,
        msg
    );
    // Logging
    info!("Encoded content: {}", content);
    return String::from(content);
}

// decode takes in a string (message), and returns it's contents;
// panics at an eventual error in decoding
pub fn decode(msg: &str) -> (String, usize) {
    // splits the message between it's header and the content before unwrapping them
    let mut msg_parts = msg.split("\r\n\r\n");
    let (header, content) = (msg_parts.next().unwrap(), msg_parts.next().unwrap());

    // get the content length from the header, remove 4 (because of the "\r\n\r\n" string) and
    // return the content parts read
    let content_length = &header["Content-Length: ".len()..].parse().unwrap();
    let content_length = content_length - "\r\n\r\n".len();

    // turn the string into BaseMessage object
    let base_message: BaseMessage = serde_json::from_str(&content[..content_length]).unwrap();
    
    // Logging
    info!("{}", serde_json::to_string(&base_message).unwrap());
    return (base_message.method, content_length);
}
