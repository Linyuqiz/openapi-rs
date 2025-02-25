use crate::common::define::{HttpFn, SD};
use crate::common::response::BaseResponse;
use crate::openapi::config::OpenApiConfig;
use crate::openapi::request::HttpBuilder;
use crate::utils;
use std::collections::HashMap;
use std::env;

#[derive(Debug, Default)]
pub struct OpenApiClient {
    config: OpenApiConfig,
    http_builder: HttpBuilder,

    headers: HashMap<String, String>,
    query_params: HashMap<String, String>,
}

impl OpenApiClient {
    pub fn new(open_api_config: OpenApiConfig) -> Self {
        let mut client = Self {
            config: open_api_config,
            ..Default::default()
        };
        client.headers = init_headers(&client.config);
        client.query_params = init_query_params(&client.config);
        client
    }

    pub fn with_request<T: SD>(&mut self, http_fn: HttpFn<T>) -> &Self {
        self.http_builder = HttpBuilder::new(http_fn);
        self.http_builder.with_base_url(&self.config.endpoint);

        for (key, value) in self.headers.iter() {
            self.http_builder.with_header(key, value);
        }
        for (key, value) in self.query_params.iter() {
            self.http_builder.with_query(key, value);
        }

        self
    }

    pub fn call<T: SD>(&self) -> anyhow::Result<BaseResponse<T>> {
        self.http_builder.build()
    }
}

fn init_headers(config: &OpenApiConfig) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    let user_id = config.user_id.clone();
    headers.insert("x-ys-user-id".to_string(), user_id);
    let x_ys_version = env::var("XYsVersion").expect("failed to get env: XYsVersion");
    headers.insert("X-Ys-Version".to_string(), x_ys_version);
    headers
}

fn init_query_params(config: &OpenApiConfig) -> HashMap<String, String> {
    let mut query_params = HashMap::new();
    let app_key = config.app_key.clone();
    query_params.insert("AppKey".to_string(), app_key);
    let timestamp = utils::time::current_timestamp().expect("failed to get timestamp");
    query_params.insert("Timestamp".to_string(), timestamp);
    query_params
}
