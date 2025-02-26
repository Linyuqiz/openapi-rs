use crate::common::define::{AsyncResponseFn, HttpFn, RequestFn};
use crate::common::request::BaseRequest;
use crate::common::response::BaseResponse;
use crate::openapi::config::OpenApiConfig;
use crate::openapi::signer::Signer;
use crate::utils;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::collections::HashMap;
use std::env;

#[derive(Debug, Default)]
pub struct OpenApiClient {
    config: OpenApiConfig,
    signer: Signer,

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

    pub async fn send<R>(&self, http_fn: HttpFn<R>) -> anyhow::Result<R>
    where
        R: std::fmt::Debug + Send + 'static,
    {
        let (req_fn, resp_fn) = http_fn();
        let request = req_fn();
        println!("发起请求到 URI: {:#?}", request);

        let mut headers = HeaderMap::new();
        for (k, v) in &self.headers {
            headers.insert(
                HeaderName::from_bytes(k.as_bytes()).expect("invalid header name"),
                HeaderValue::from_str(v).expect("invalid header value"),
            );
        }

        let url = format!(
            "{}{}?{}",
            self.config.endpoint,
            request.uri,
            serde_urlencoded::to_string(&self.query_params).expect("failed to encode query params")
        );

        let response = reqwest::Client::new()
            .get(&url)
            .headers(headers)
            .form(&self.query_params)
            .send()
            .await
            .expect("failed to send request");

        // 调用异步响应解析回调，并 await 其结果
        let parsed_response = resp_fn(response).await?;
        Ok(parsed_response)
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
