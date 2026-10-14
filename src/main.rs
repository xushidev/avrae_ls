// use standard in/out
use std::io;
use std::io::prelude::*;

// environment variables and path
use std::env;

use log::info;
use serde_json::{Result, Value::Null, json};
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
            "{}/{}",
            env::home_dir().unwrap().display(),
            "avrae_ls.log"
        ))
        .unwrap(),
    )])
    .unwrap();
    info!("Successfully inizialised logger.");

    // Begin the message handling
    let mut stdin = io::stdin();
    loop {
        // get all the daout
        let mut buffer = String::new();
        stdin.read_to_string(&mut buffer).unwrap();

        // pass all the data to the decoder
        let request_message = rpc::decode(&buffer);

        // Logging the method and id
        info!(
            "Received message with method: {} and id: {}",
            request_message.method, request_message.id
        );
        match handle_messages(request_message) {
            Err(error) => break,
            Ok(value) => info!("Successfully handled the message"),
        };
    }

    return Ok(());
}

fn handle_messages(msg: RequestMessage) -> Result<()> {
    let mut stdout = io::stdout();
    if msg.method == "inizialize" {
        // Building inizialize response message
        let initialize_params: InitializeResult = InitializeResult {
            // server capabilities:
            capabilities: json!({}),
            // server info defined as {name: string, version?: string}
            serverInfo: json!({"name": "avrae_ls", "version": "1.0"}).into(),
        };

        let response_message: ResponseMessage = ResponseMessage {
            id: Some(msg.id),
            result: Some(serde_json::value::to_value(initialize_params).unwrap()),
            error: Some(Null),
        };

        stdout
            .write(serde_json::to_string(&response_message).unwrap().as_bytes())
            .unwrap();
        return Ok(());
    }
    return Ok(());
}
