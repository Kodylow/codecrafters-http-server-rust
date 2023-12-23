pub struct Response {
    status_code: &'static str,
    content_type: &'static str,
    body: String,
}

impl Response {
    pub fn new(status_code: &'static str, content_type: &'static str, body: String) -> Self {
        Response {
            status_code,
            content_type,
            body,
        }
    }

    pub fn format(&self) -> String {
        format!(
            "HTTP/1.1 {}\r\nContent-Type: {}\r\n\r\n{}",
            self.status_code, self.content_type, self.body
        )
    }
}
