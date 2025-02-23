use crate::openapi::client_http::HttpClient;
use crate::openapi::config::OpenApiConfig;
use crate::utils;
use std::collections::HashMap;
use std::env;

#[derive(Default, Debug)]
pub struct OpenApiClient {
    client: HttpClient,
    config: OpenApiConfig,

    headers: HashMap<&'static str, &'static str>,
    query_params: HashMap<&'static str, &'static str>,
}

impl OpenApiClient {
    pub fn new(open_api_config: OpenApiConfig) -> Self {
        let mut client = Self {
            config: open_api_config,
            ..Default::default()
        };
        client.init_headers();
        client.init_query_params();
        client
    }

    fn init_headers(&mut self) {
        let mut headers = HashMap::<&'static str, &'static str>::new();
        headers.insert("x-ys-user-id", &self.config.user_id);
        let x_ys_version = env::var("XYsVersion").expect("failed to get env: XYsVersion");
        headers.insert("X-Ys-Version", &x_ys_version);
        self.headers = headers;
    }

    fn init_query_params(&mut self) {
        let mut query_params = HashMap::new();
        query_params.insert("AppKey", &*self.config.app_key);
        let timestamp = utils::time::current_timestamp().expect("failed to get timestamp");
        query_params.insert("Timestamp", &*timestamp);
        self.query_params = query_params;
    }

    pub fn with_header(&mut self, key: &'static str, value: &'static str) {
        self.headers.insert(key, value);
    }

    pub fn with_query_param(&mut self, key: &'static str, value: &'static str) {
        self.query_params.insert(key, value);
    }
}
