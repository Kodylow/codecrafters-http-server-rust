// Uncomment this block to pass the first stage
use anyhow::{bail, Result};
use std::net::TcpListener;
use utils::{handle_connection, parse_directory_from_cli, tracing_setup};

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
