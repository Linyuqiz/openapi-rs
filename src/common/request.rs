use crate::common::define::BaseRequest;
use reqwest::{Client, Method, RequestBuilder};

#[derive(Debug, Default)]
pub struct HttpBuilder {
    pub http_client: Client,
    pub base_url: String,

    pub base_request: BaseRequest,
}

impl HttpBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = base_url;
        self
    }

    pub fn with_base_request(mut self, base_request: BaseRequest) -> Self {
        self.base_request = base_request;
        self
    }

    pub fn builder(self) -> anyhow::Result<RequestBuilder> {
        let url = format!(
            "{}{}?{}",
            self.base_url,
            self.base_request.uri,
            serde_urlencoded::to_string(&self.base_request.queries)?
        );

        match self.base_request.method {
            Method::GET => Ok(self
                .http_client
                .get(&url)
                .headers(self.base_request.headers.clone())),
            Method::POST => Ok(self
                .http_client
                .post(&url)
                .headers(self.base_request.headers.clone())
                .body(self.base_request.body)),
            _ => Err(anyhow::anyhow!("unsupported method")),
        }
    }
}
