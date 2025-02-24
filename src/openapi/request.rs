use crate::common::define::{HttpFn, RequestFn, ResponseFn};
use crate::common::response::BaseResponse;
use reqwest::Body;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug)]
pub struct HttpBuilder {
    method: reqwest::Method,
    base_url: String,
    base_uri: String,

    headers: Option<HashMap<String, String>>,

    query: Option<HashMap<String, String>>,
    form: Option<HashMap<String, String>>,
    body: Option<Body>,
}

impl HttpBuilder {
    pub fn new<T: Serialize, U: Deserialize>(http_fn: HttpFn<T, U>) -> Self {
        let (request_fn, response_fn) = http_fn();
        let request = request_fn();
        Self {
            method: request.method,
            base_url: "".to_string(),
            base_uri: request.uri,
            headers: request.headers,
            query: request.query,
            form: request.form,
            body: request.body.map(|body| Body::from(body)),
        }
    }

    pub fn with_method(&mut self, method: reqwest::Method) -> &Self {
        self.method = method;
        self
    }

    pub fn with_base_url(&mut self, base_url: &str) -> &Self {
        self.base_url = base_url.to_string().into();
        self
    }

    pub fn with_base_uri(&mut self, base_uri: &str) -> &Self {
        self.base_uri = base_uri.to_string().into();
        self
    }

    pub fn with_header(&mut self, key: &str, value: &str) -> &Self {
        if let Some(ref mut headers) = self.headers {
            headers.insert(key.to_string(), value.to_string());
        } else {
            self.headers = Some(HashMap::from([(key.to_string(), value.to_string())]));
        }
        self
    }

    pub fn with_query(&mut self, key: &str, value: &str) -> &Self {
        if let Some(ref mut query) = self.query {
            query.insert(key.to_string(), value.to_string());
        } else {
            self.query = Some(HashMap::from([(key.to_string(), value.to_string())]));
        }
        self
    }

    pub fn with_form(&mut self, key: &str, value: &str) -> &Self {
        if let Some(ref mut form) = self.form {
            form.insert(key.to_string(), value.to_string());
        } else {
            self.form = Some(HashMap::from([(key.to_string(), value.to_string())]));
        }
        self
    }

    pub fn with_body(&mut self, body: &Body) -> &Self {
        if let Some(ref mut body) = self.body {
            *body = body.clone();
        } else {
            self.body = Some(body);
        }
        self
    }

    pub fn build<U: Deserialize>(&self) -> anyhow::Result<BaseResponse<U>> {
        // serde_urlencoded::to_string(&query_params).expect("failed to encode query params");
        unimplemented!("not implemented yet")
    }
}
