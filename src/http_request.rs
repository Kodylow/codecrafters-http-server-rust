use anyhow::{anyhow, Context, Result};

pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub version: String,
    pub host: String,
    pub user_agent: String,
}

impl HttpRequest {
    pub fn parse(request: &str) -> Result<HttpRequest> {
        let mut lines = request.lines();

        let (method, path, version) =
            Self::parse_start_line(lines.next().ok_or_else(|| anyhow!("Request is empty"))?)?;
        let host = Self::parse_header(&mut lines, "Host:")?;
        let user_agent = Self::parse_header(&mut lines, "User-Agent:")?;

        Ok(HttpRequest {
            method,
            path,
            version,
            host,
            user_agent,
        })
    }

    fn parse_start_line(line: &str) -> Result<(String, String, String)> {
        let mut parts = line.split_whitespace();
        let method = parts.next().context("Method not found")?.to_string();
        let path = parts.next().context("Path not found")?.to_string();
        let version = parts.next().context("Version not found")?.to_string();
        Ok((method, path, version))
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
