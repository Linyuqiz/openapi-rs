use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct RequestBuilder {
    client: reqwest::Client,

    base_url: String,
    route: String,
    method: reqwest::Method,
}

impl RequestBuilder {
    pub fn new(client: reqwest::Client) -> Self {
        Self {
            client,
            ..Default::default()
        }
    }

    pub fn with_base_url(&mut self, base_url: &str) -> &Self {
        self.base_url = base_url.to_string();
        self
    }

    pub fn with_route(&mut self, route: &str) -> &Self {
        self.route = route.to_string();
        self
    }

    pub fn with_method(&mut self, method: reqwest::Method) -> &Self {
        self.method = method;
        self
    }

    pub fn with_query_param(&mut self, query_params: HashMap<String, String>) -> &Self {
        let query_params_content =
            serde_urlencoded::to_string(&query_params).expect("failed to encode query params");
        self.route = format!("{}?{}", self.route, query_params_content);
        self
    }
}
