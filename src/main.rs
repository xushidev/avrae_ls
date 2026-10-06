// use standard in/out
use std::io;
use std::io::prelude::*;

use serde_json::json;
use simplelog::{CombinedLogger, Config, LevelFilter, WriteLogger};
use std::fs::File;

// import the rpc.rs, structs.rs files as module
mod rpc;
mod structs;

fn main() -> io::Result<()> {
    // Inizialise the logger
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Info,
        Config::default(),
        File::create("avrae_ls.log").unwrap(),
    )])
    .unwrap();

    // stdio requires to send out messages as bytes instead of strings
    // io::stdout().write("hello world\n".as_bytes())?;

    // test case
    let req = json!({
        "jsonrpc": "2.0",
        "id": 23,
        "method": "true",
        "params": {}
    });

    // encode the json {} using the rpc::encode function
    let content = rpc::encode("{\"method\": \"true\"}");

    let new_content = rpc::encode(&req.to_string());

    // debug out
    // io::stdout().write(content.as_bytes())?;
    let (method, length) = rpc::decode(&content);

    return Ok(());
}
