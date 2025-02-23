use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct HttpClient {
    http_client_builder: reqwest::ClientBuilder,
}

impl HttpClient {
    pub fn new() -> Self {
        Self {
            http_client_builder: reqwest::ClientBuilder::new(),
        }
    }

    pub fn build(self) -> reqwest::Client {
        self.http_client_builder
            .build()
            .expect("failed to build http client")
    }

    pub fn with_headers(mut self, headers: HashMap<&str, &str>) -> Self {
        let mut http_headers = HeaderMap::new();
        for (key, value) in headers {
            http_headers.insert(
                HeaderName::from_bytes(key.as_bytes()).expect("invalid header name"),
                HeaderValue::from_str(value).expect("invalid header value"),
            );
        }
        self.http_client_builder = self.http_client_builder.default_headers(http_headers);
        self
    }
}
