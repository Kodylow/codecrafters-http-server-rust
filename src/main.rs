// Uncomment this block to pass the first stage
use anyhow::{bail, Result};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::{env, fs};
use utils::tracing_setup;

use tracing::info;

mod http;
mod utils;
use http::{Request, Response};

// To try this locally on macOS:
// run ./your_server.sh in one terminal session
// nc -vz 127.0.0.1 4221 in another.
// (-v gives more verbose output,
// -z just scan for listening daemons, without sending any data to them.)

const OK: &str = "200 OK";
const NOT_FOUND: &str = "404 NOT FOUND";
const TEXT_PLAIN: &str = "text/plain";

fn router(request: Request, directory: &str) -> Response {
    match request.path.as_str() {
        path if path.starts_with("/echo/") => {
            let random_string = &path[6..];
            Response::new(OK, TEXT_PLAIN, random_string.to_string())
        }
        path if path.starts_with("/files/") => {
            let filename = &path[7..];
            let filepath = format!("{}/{}", directory, filename);
            match fs::read_to_string(&filepath) {
                Ok(contents) => Response::new(OK, "application/octet-stream", contents),
                Err(_) => Response::new(NOT_FOUND, TEXT_PLAIN, "".to_string()),
            }
        }
        path if path == "/" || path == "/user-agent" => {
            let content =
                if path == "/" {
                    "".to_string()
                } else {
                    request.user_agent.clone()
                };
            Response::new(OK, TEXT_PLAIN, content)
        }
        _ => Response::new(NOT_FOUND, TEXT_PLAIN, "".to_string()),
    }
}

fn parse_stream(stream: &mut std::net::TcpStream) -> Result<String> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8(buffer.to_vec())?;
    info!("request stream: {}", request);
    Ok(request)
}

fn handle_connection(mut stream: std::net::TcpStream, directory: String) {
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

fn main() -> Result<()> {
    tracing_setup()?;

    let listener = TcpListener::bind("127.0.0.1:4221")?;

    let args: Vec<String> = env::args().collect();
    let directory = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| String::from("default_directory"));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let directory = directory.clone();
                std::thread::spawn(move || handle_connection(stream, directory));
            }
            Err(e) => {
                bail!("Unable to connect: {}", e);
            }
        }
    }

    Ok(())
}
