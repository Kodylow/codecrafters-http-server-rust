// Uncomment this block to pass the first stage
use anyhow::{bail, Result};
use std::io::{Read, Write};
use std::net::TcpListener;

use tracing::info;

mod http_request;
use http_request::HttpRequest;

// To try this locally on macOS:
// run ./your_server.sh in one terminal session
// nc -vz 127.0.0.1 4221 in another.
// (-v gives more verbose output,
// -z just scan for listening daemons, without sending any data to them.)

const RESPONSE_200: &str = "HTTP/1.1 200 OK\r\n\r\n";
const RESPONSE_404: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

fn main() -> Result<()> {
    tracing_setup()?;

    let listener = TcpListener::bind("127.0.0.1:4221")?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let request = parse_stream(&mut stream)?;

                let parsed_request = HttpRequest::parse(&request)?;

                let response = match parsed_request.path.as_str() {
                    "/" => RESPONSE_200,
                    _ => RESPONSE_404,
                };

                stream.write(response.as_bytes()).unwrap();
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
