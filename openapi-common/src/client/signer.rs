use crate::define::BaseRequest;
use openapi_util::encrypt::md5::md5;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Write;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Signer {
    app_key: String,
    app_secret: String,
}

impl Signer {
    pub fn new(app_key: &str, app_secret: &str) -> Self {
        Self {
            app_key: app_key.to_string(),
            app_secret: app_secret.to_string(),
        }
    }

    pub fn sign_request(
        &self,
        base_request: &BaseRequest,
        query_params: &HashMap<String, String>,
    ) -> anyhow::Result<String> {
        let mut query_params = query_params.clone();
        Ok(self
            .sign(&mut query_params)
            .expect("sign query params failed"))
    }

    pub fn sign(&self, query_params: &HashMap<String, String>) -> anyhow::Result<String> {
        let mut buffer = String::new();
        let mut keys: Vec<String> = query_params.keys().cloned().collect();
        keys.sort();

        for key in keys {
            if let Some(val) = query_params.get(&key) {
                write!(buffer, "{}={}", key, val).expect("failed to write buffer");
            }
        }
        buffer.push_str(&self.app_secret);

        Ok(md5(&buffer))
    }
}
