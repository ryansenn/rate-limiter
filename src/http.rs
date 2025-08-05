
// just strings for now
pub struct HttpRequest {
    pub request: String,
    pub headers: String,
    pub body: String,
}

impl From<&[u8]> for HttpRequest {
   fn from(buf: &[u8]) -> Self {
        let raw = std::str::from_utf8(buf).unwrap_or("");
        let mut parts = raw.split("\r\n\r\n");
        let head = parts.next().unwrap_or("");
        let body = parts.next().unwrap_or("").to_string();

        let mut head_parts = head.split("\r\n");
        let request = head_parts.next().unwrap_or("").to_string();
        let headers = head_parts.collect();

        Self {request, headers, body}
   }
}

pub struct HttpResponse {
    version: String,
    status_code: u16,
    reason_phrase: String, 
    headers: String,
    body: String,
}

