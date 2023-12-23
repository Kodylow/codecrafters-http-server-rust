// Uncomment this block to pass the first stage
use anyhow::{bail, Result};
use std::io::{Read, Write};
use std::net::TcpListener;

use tracing::info;

mod http;
use http::{Request, Response};

// To try this locally on macOS:
// run ./your_server.sh in one terminal session
// nc -vz 127.0.0.1 4221 in another.
// (-v gives more verbose output,
// -z just scan for listening daemons, without sending any data to them.)

const OK: &str = "200 OK";
const NOT_FOUND: &str = "404 NOT FOUND";
const TEXT_PLAIN: &str = "text/plain";

fn handle_connection(mut stream: std::net::TcpStream) {
    let request = parse_stream(&mut stream).unwrap();
    let parsed_request = Request::parse(&request).unwrap();
    let response = create_response(parsed_request);
    stream.write(response.format().as_bytes()).unwrap();
}

fn main() -> Result<()> {
    tracing_setup()?;

    let listener = TcpListener::bind("127.0.0.1:4221")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || handle_connection(stream));
            }
            Err(e) => {
                bail!("Unable to connect: {}", e);
            }
        }
    }

    Ok(())
}

fn tracing_setup() -> Result<(), anyhow::Error> {
    let tracer = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(tracer)?;
    Ok(())
}

fn parse_stream(stream: &mut std::net::TcpStream) -> Result<String> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8(buffer.to_vec())?;
    info!("request stream: {}", request);
    Ok(request)
}

fn create_response(request: Request) -> Response {
    match request.path.as_str() {
        path if path.starts_with("/echo/") => {
            let random_string = &path[6..];
            Response::new(OK, TEXT_PLAIN, random_string.to_string())
        }
        path if path == "/" => Response::new(OK, TEXT_PLAIN, "".to_string()),
        path if path == "/user-agent" => Response::new(OK, TEXT_PLAIN, request.user_agent),
        _ => Response::new(NOT_FOUND, TEXT_PLAIN, "".to_string()),
    }
}
