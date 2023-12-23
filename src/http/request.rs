use anyhow::{anyhow, Context, Result};

pub struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
    pub host: String,
    pub user_agent: String,
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
        let host = Self::parse_header(&mut lines, "Host:")?;
        let user_agent = Self::parse_header(&mut lines, "User-Agent:")?;

        Ok(Request {
            method: start_line.method,
            path: start_line.path,
            version: start_line.version,
            host,
            user_agent,
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
    ) -> Result<String> {
        let line = lines
            .find(|&line| line.starts_with(header))
            .ok_or_else(|| anyhow!("Header not found"))?;
        line.split_whitespace()
            .nth(1)
            .context("Header value not found")
            .map(|s| s.to_string())
    }
}
