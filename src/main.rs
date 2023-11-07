// Uncomment this block to pass the first stage
use std::io::Write;
use std::net::TcpListener;

use tracing::{error, info};

// To try this locally on macOS:
// run ./your_server.sh in one terminal session
// nc -vz 127.0.0.1 4221 in another.
// (-v gives more verbose output,
// -z just scan for listening daemons, without sending any data to them.)

fn main() {
    let tracer = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(tracer).unwrap();
    // info!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                // read data from connection
                let mut buf = [0; 512];
                stream.peek(&mut buf).unwrap();
                // log the data
                info!("Received data: {}", String::from_utf8_lossy(&buf[..]));
                // send response HTTP/1.1 2002 OK\r\n\r\n
                let response = "HTTP/1.1 200 OK\r\n\r\n";
                if let Err(e) = stream.write(response.as_bytes()) {
                    error!("error: {}", e);
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
