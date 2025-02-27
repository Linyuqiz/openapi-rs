use crate::client::config::OpenApiConfig;
use crate::client::request::HttpBuilder;
use crate::client::signer::Signer;
use crate::define::HttpFn;
use anyhow::anyhow;
use openapi_util::time::time::current_timestamp;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::collections::HashMap;
use std::env;

pub mod config;
pub mod request;
pub mod signer;

#[derive(Debug, Default)]
pub struct OpenApiClient {
    config: OpenApiConfig,
    signer: Signer,
}

impl OpenApiClient {
    pub fn new(open_api_config: OpenApiConfig) -> Self {
        let app_key = open_api_config.app_key.clone();
        let app_secret = open_api_config.app_secret.clone();
        Self {
            config: open_api_config,
            signer: Signer::new(&app_key, &app_secret),
            ..Default::default()
        }
    }

    pub async fn send<R>(&mut self, http_fn: HttpFn<R>) -> anyhow::Result<R>
    where
        R: std::fmt::Debug + Send + 'static,
    {
        let default_headers = init_headers(&self.config)?;
        let mut default_query_params = init_query_params(&self.config)?;

        let (req_fn, resp_fn) = http_fn();
        let request = req_fn();

        let mut headers = HeaderMap::new();
        for (k, v) in default_headers {
            headers.insert(
                HeaderName::from_bytes(k.as_bytes())?,
                HeaderValue::from_str(&*v)?,
            );
        }
        let signature = self.signer.sign_request(&request, &default_query_params)?;
        default_query_params.insert("Signature".to_string(), signature);
        let url = format!(
            "{}{}?{}",
            self.config.endpoint,
            request.uri,
            serde_urlencoded::to_string(&default_query_params)?
        );

        let response = HttpBuilder::new()
            .builder()
            .get(&url)
            .headers(headers)
            .form(&default_query_params)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "failed to send request: {}",
                response.status().to_string()
            ));
        }

        Ok(resp_fn(response).await?)
    }
}

fn init_headers(config: &OpenApiConfig) -> anyhow::Result<HashMap<String, String>> {
    let mut headers = HashMap::new();
    let user_id = config.user_id.clone();
    headers.insert("x-ys-user-id".to_string(), user_id);
    let x_ys_version = env::var("XYsVersion")?;
    headers.insert("X-Ys-Version".to_string(), x_ys_version);
    Ok(headers)
}

fn init_query_params(config: &OpenApiConfig) -> anyhow::Result<HashMap<String, String>> {
    let mut query_params = HashMap::new();
    let app_key = config.app_key.clone();
    query_params.insert("AppKey".to_string(), app_key);
    let timestamp = current_timestamp()?;
    query_params.insert("Timestamp".to_string(), timestamp);
    Ok(query_params)
}
