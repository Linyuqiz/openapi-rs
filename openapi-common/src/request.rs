use reqwest::Client;

#[derive(Debug, Default)]
pub struct HttpBuilder {
    pub http_client: Client,
}

impl HttpBuilder {
    pub fn new() -> Self {
        HttpBuilder {
            http_client: Client::new(),
        }
    }

    pub fn builder(self) -> Client {
        self.http_client
    }
}
