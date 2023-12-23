use std::fs;

use tracing::info;

use crate::http::{Request, Response};

const OK: &str = "200 OK";
const NOT_FOUND: &str = "404 NOT FOUND";
const TEXT_PLAIN: &str = "text/plain";
const OCTET_STREAM: &str = "application/octet-stream";

pub fn router(request: Request, directory: &str) -> Response {
    match request.path.as_str() {
        path if path.starts_with("/echo/") => handle_echo(path),
        path if path.starts_with("/files/") => handle_file(path, directory),
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

fn handle_echo(path: &str) -> Response {
    let random_string = &path[6..];
    Response::new(OK, TEXT_PLAIN, random_string.to_string())
}

fn handle_file(path: &str, directory: &str) -> Response {
    let filename = &path[7..];
    let filepath = format!("{}/{}", directory, filename);
    info!("filepath: {}", filepath);
    match fs::read_to_string(&filepath) {
        Ok(contents) => Response::new(OK, OCTET_STREAM, contents),
        Err(_) => Response::new(NOT_FOUND, TEXT_PLAIN, "".to_string()),
    }
}
