use std::{
    env,
    io::{Read, Write},
};

use crate::{http::Request, router};
use anyhow::Result;

pub fn tracing_setup() -> Result<(), anyhow::Error> {
    let tracer = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(tracer)?;
    Ok(())
}

pub fn parse_stream(stream: &mut std::net::TcpStream) -> Result<String> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8(buffer.to_vec())?;
    Ok(request)
}

pub fn handle_connection(mut stream: std::net::TcpStream, directory: String) {
    let request = parse_stream(&mut stream).unwrap();
    match Request::parse(&request) {
        Ok(parsed_request) => {
            let response = router(parsed_request, &directory);
            stream.write(response.format().as_bytes()).unwrap();
        }
        Err(e) => {
            eprintln!("Failed to parse request: {}", e);
            // Handle the error appropriately here
        }
    }
}

pub fn parse_directory_from_cli() -> String {
    let args: Vec<String> = env::args().collect();
    let directory_arg_position = args.iter().position(|arg| arg == "--directory");

    match directory_arg_position {
        Some(index) => {
            if index < args.len() - 1 {
                args[index + 1].clone()
            } else {
                String::from("default_directory")
            }
        }
        None => String::from("default_directory"),
    }
}
