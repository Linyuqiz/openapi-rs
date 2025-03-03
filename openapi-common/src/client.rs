use crate::config::{EndpointType, OpenApiConfig};
use crate::define::{BaseRequest, HttpFn};
use crate::request::HttpBuilder;
use crate::signer::Signer;
use anyhow::anyhow;
use openapi_util::time::current_timestamp;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::collections::HashMap;
use std::env;

#[derive(Debug, Default)]
pub struct OpenApiClient {
    config: OpenApiConfig,
    signer: Signer,

    endpoint_type: EndpointType,
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

    pub fn with_endpoint_type(mut self, endpoint_type: EndpointType) -> Self {
        self.endpoint_type = endpoint_type;
        self
    }

    pub async fn send<R>(&mut self, http_fn: HttpFn<R>) -> anyhow::Result<R>
    where
        R: std::fmt::Debug + Send + 'static,
    {
        let (req_fn, resp_fn) = http_fn();
        let mut base_request = req_fn();

        self.default_headers_queries(&mut base_request)?;

        let endpoint = match self.endpoint_type {
            EndpointType::Api => self.config.endpoint.clone(),
            EndpointType::Cloud => self.config.cloud_endpoint.clone(),
            EndpointType::Hpc => self.config.hpc_endpoint.clone(),
            EndpointType::Sync => self.config.sync_endpoint.clone(),
        };

        let response = HttpBuilder::new()
            .with_base_url(endpoint)
            .with_base_request(base_request)
            .builder()?
            .send()
            .await?;

        dbg!(&response);

        if !response.status().is_success() {
            return Err(anyhow!(
                "failed to send request: {}",
                response.status().to_string()
            ));
        }

        resp_fn(response).await
    }

    fn default_headers_queries(
        &mut self,
        base_request: &mut BaseRequest,
    ) -> Result<(), anyhow::Error> {
        let mut headers = HeaderMap::new();
        for (k, v) in default_headers(&self.config)? {
            headers.insert(
                HeaderName::from_bytes(k.as_bytes())?,
                HeaderValue::from_str(&*v)?,
            );
        }
        base_request.headers.iter().for_each(|(k, v)| {
            headers.insert(k, v.clone());
        });

        let mut default_queries = default_queries(&self.config)?;
        if let Some(ref queries) = base_request.queries {
            queries.iter().for_each(|(k, v)| {
                default_queries.insert(k.to_string(), v.to_string());
            });
        }

        // signature
        default_queries.insert(
            "Signature".to_string(),
            self.signer.sign_request(base_request, &default_queries)?,
        );

        base_request.headers = headers.clone();
        base_request.queries = Some(default_queries.clone());

        Ok(())
    }
}

fn default_headers(config: &OpenApiConfig) -> anyhow::Result<HashMap<String, String>> {
    let mut headers = HashMap::new();
    let user_id = config.user_id.clone();
    headers.insert("x-ys-user-id".to_string(), user_id);
    let x_ys_version = env::var("XYsVersion")?;
    headers.insert("X-Ys-Version".to_string(), x_ys_version);
    Ok(headers)
}

fn default_queries(config: &OpenApiConfig) -> anyhow::Result<HashMap<String, String>> {
    let mut query_params = HashMap::new();
    let app_key = config.app_key.clone();
    query_params.insert("AppKey".to_string(), app_key);
    let timestamp = current_timestamp()?;
    query_params.insert("Timestamp".to_string(), timestamp);
    Ok(query_params)
}
