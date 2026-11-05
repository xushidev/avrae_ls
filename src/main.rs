// use standard in/out
use std::io;
use std::io::prelude::*;

// environment variables and path
use std::env;

use log::info;
use serde_json::json;
use simplelog::{CombinedLogger, Config, LevelFilter, WriteLogger};
use std::fs::File;

use crate::structs::{InitializeResult, RequestMessage, ResponseMessage};

// import the rpc.rs, structs.rs files as module
mod rpc;
mod structs;

fn main() -> io::Result<()> {
    // Inizialise the logger
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Info,
        Config::default(),
        File::create(format!(
            "{}/.local/state/nvim/{}",
            env::home_dir().unwrap().display(),
            "avrae_ls.log"
        ))
        .unwrap(),
    )])
    .unwrap();

    // Begin the message handling
    let stdin = io::stdin();
    loop {
        // Every loop handles a message Header\r\n\r\nBody
        let mut handle = stdin.lock();
        let mut buffer = vec![];
        let bytes_read = handle.read_until(b'\n', &mut buffer).unwrap();

        if bytes_read == 0 {
            // if zero bytes are read, then that means we have reached EOF and
            // we can get out of the loop
            break;
        }
        // Get the byte size by using string manipulation
        let byte_size: usize = String::from_utf8(buffer)
            .unwrap()
            .strip_suffix("\r\n")
            .unwrap()
            .split_whitespace() // ["Content-Length:", "byte_size"]
            .next_back() // Gets the string from the end: byte_size
            .unwrap()
            .parse() // Parse it into an integer
            .unwrap();

        let mut body_buffer = vec![0u8; byte_size + 2];
        handle.read_exact(&mut body_buffer).unwrap();
        // Reads the entire body with \r\n at the beginning that we can remove with strip_prefix
        let body: String = String::from(
            String::from_utf8(body_buffer)
                .unwrap()
                .strip_prefix("\r\n")
                .unwrap(),
        );

        // Deserialize the body into an actual request message
        let request_message: RequestMessage = serde_json::from_str(&body).unwrap();
        handle_messages(request_message);
    }

    return Ok(());
}

fn handle_messages(msg: RequestMessage) {
    let mut stdout = io::stdout().lock();
    match msg.method.as_str() {
        "initialize" => {
            // Building inizialize response message
            let initialize_params: InitializeResult = InitializeResult {
                // server capabilities:
                capabilities: json!({
                    "hoverProvider": true,
                    "textDocumentSync": 1,
                    "definitionProvider": true
                }),
                // server info defined as {name: string, version?: string}
                serverInfo: json!({"name": "avrae_ls", "version": "1.0"}).into(),
            };

            let response_message: ResponseMessage = ResponseMessage {
                jsonrpc: "2.0".to_string(),
                id: msg.id,
                result: Some(serde_json::value::to_value(initialize_params).unwrap()),
                error: None,
            };

            stdout
                .write_all(
                    rpc::encode(&serde_json::to_string(&response_message).unwrap()).as_bytes(),
                )
                .unwrap();
            stdout.flush().unwrap();
        }
        "initialized" => {
            info!("Server initialized successfully");
        }
        "shutdown" => {
            info!("Server shutdown");
            std::process::exit(0);
        }
        "textDocument/hover" => {
            info!("Hover request received!");

            let response_message: ResponseMessage = ResponseMessage {
                jsonrpc: "2.0".to_string(),
                id: msg.id,
                result: Some(json!({
                    "contents": "Hello from Language Server!",
                })),
                error: None,
            };

            stdout
                .write_all(
                    rpc::encode(&serde_json::to_string(&response_message).unwrap()).as_bytes(),
                )
                .unwrap();
            stdout.flush().unwrap();
        }
        _ => {
            // If it doesn't matches anything, simply log it
            info!("Got a {} method, which is not implemented", msg.method);
        }
    }
}
