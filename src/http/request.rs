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
        let host = Self::parse_header(&mut lines, "Host:").unwrap_or_default();
        let user_agent = Self::parse_header(&mut lines, "User-Agent:").unwrap_or_default();
        let body = lines.collect::<Vec<&str>>().join("\n");

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

    fn parse_header<'a, I: Iterator<Item = &'a str>>(
        lines: &mut I,
        header: &str,
    ) -> Option<String> {
        let line = lines.find(|&line| line.starts_with(header))?;
        line.split_whitespace().nth(1).map(|s| s.to_string())
    }
}
