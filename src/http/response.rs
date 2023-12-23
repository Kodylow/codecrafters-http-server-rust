pub struct Response {
    status_code: &'static str,
    content_type: &'static str,
    content_length: usize,
    body: String,
}

impl Response {
    pub fn new(status_code: &'static str, content_type: &'static str, body: String) -> Self {
        Response {
            status_code,
            content_type,
            content_length: body.len(),
            body,
        }
    }

    pub fn format(&self) -> String {
        format!(
            "{}Content-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
            self.status_code, self.content_type, self.content_length, self.body
        )
    }
}
