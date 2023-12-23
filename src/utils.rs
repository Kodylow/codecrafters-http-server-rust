use std::io::{Read, Write};

use crate::{http::Request, router};
use anyhow::Result;
use tracing::info;

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
    info!("request stream: {}", request);
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
