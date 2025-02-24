use crate::common::define::HttpFn;
use crate::common::response::BaseResponse;
use crate::openapi::config::OpenApiConfig;
use crate::openapi::request::HttpBuilder;
use crate::utils;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

#[derive(Debug)]
pub struct OpenApiClient {
    config: OpenApiConfig,
    http_builder: HttpBuilder,

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

    pub fn with_request<T: Serialize, U: Deserialize>(&mut self, http_fn: HttpFn<T, U>) -> &Self {
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

    pub fn call<U: Deserialize>(&self) -> anyhow::Result<BaseResponse<U>> {
        self.http_builder.build()
    }
}
