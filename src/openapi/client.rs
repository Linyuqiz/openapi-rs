use crate::common::define::{AsyncResponseFn, HttpFn, RequestFn};
use crate::common::request::BaseRequest;
use crate::common::response::BaseResponse;
use crate::openapi::config::OpenApiConfig;
use crate::utils;
use std::collections::HashMap;
use std::env;

#[derive(Debug, Default)]
pub struct OpenApiClient {
    config: OpenApiConfig,

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

    pub async fn send<T, R>(&self, http_fn: HttpFn<T, R>) -> anyhow::Result<R>
    where
        T: std::fmt::Debug + Send + 'static,
        R: std::fmt::Debug + Send + 'static,
    {
        // 生成请求对象
        let (req_fn, resp_fn) = http_fn();
        let request = req_fn();
        println!("发起请求到 URI: {:#?}", request);

        // 使用异步 reqwest 客户端发送请求
        let client = reqwest::Client::new();
        // 示例中使用 httpbin.org 来模拟请求（实际可以使用 request.uri() 组合完整 URL）
        let response = client.get("http://httpbin.org/get").send().await?;

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
