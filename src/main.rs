// Uncomment this block to pass the first stage
use anyhow::{bail, Result};
use std::env;
use std::net::TcpListener;
use utils::{handle_connection, tracing_setup};

mod http;
mod router;
mod utils;
use router::router;

// To try this locally on macOS:
// run ./your_server.sh in one terminal session
// nc -vz 127.0.0.1 4221 in another.
// (-v gives more verbose output,
// -z just scan for listening daemons, without sending any data to them.)

fn main() -> Result<()> {
    tracing_setup()?;

    let listener = TcpListener::bind("127.0.0.1:4221")?;

    let directory = parse_directory_from_cli();

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

fn parse_directory_from_cli() -> String {
    let args: Vec<String> = env::args().collect();
    args.iter()
        .find(|arg| arg.starts_with("--directory="))
        .map(|arg| {
            arg.splitn(2, '=')
                .nth(1)
                .unwrap_or("default_directory")
                .to_string()
        })
        .unwrap_or_else(|| String::from("default_directory"))
}
