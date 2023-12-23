use anyhow::{anyhow, Context, Result};

pub struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
    pub host: String,
    pub user_agent: String,
    pub body: String,
}

pub struct StartLine {
    pub method: String,
    pub path: String,
    pub version: String,
}

impl Request {
    pub fn parse(request: &str) -> Result<Request> {
        let mut lines = request.lines();
        let start_line =
            Self::parse_start_line(lines.next().ok_or_else(|| anyhow!("Request is empty"))?)?;
        let mut host = String::new();
        let mut user_agent = String::new();
        let mut body = String::new();
        let mut body_started = false;

        for line in lines {
            if !body_started {
                if line.starts_with("Host:") {
                    host = Self::parse_header(line, "Host:").unwrap_or_default();
                } else if line.starts_with("User-Agent:") {
                    user_agent = Self::parse_header(line, "User-Agent:").unwrap_or_default();
                } else if line.is_empty() {
                    body_started = true;
                }
            } else {
                body.push_str(line);
                body.push('\n');
            }
        }

        Ok(Request {
            method: start_line.method,
            path: start_line.path,
            version: start_line.version,
            host,
            user_agent,
            body,
        })
    }

    fn parse_start_line(line: &str) -> Result<StartLine> {
        let mut parts = line.split_whitespace();
        let method = parts.next().context("Method not found")?.to_string();
        let path = parts.next().context("Path not found")?.to_string();
        let version = parts.next().context("Version not found")?.to_string();

        Ok(StartLine {
            method,
            path,
            version,
        })
    }

    fn parse_header(line: &str, header: &str) -> Option<String> {
        if line.starts_with(header) {
            line.split_whitespace().nth(1).map(|s| s.to_string())
        } else {
            None
        }
    }
}
