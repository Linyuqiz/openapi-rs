use crate::define::BaseRequest;
use openapi_util::md5::md5;
use openapi_util::sha1::sha1;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Write;
use std::str::from_utf8;

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
        queries: &mut HashMap<String, String>,
    ) -> anyhow::Result<String> {
        if !base_request.body.is_empty() {
            if let Some(content_type) = &base_request.content_type {
                if content_type.as_str() == "application/json" {
                    queries.insert("_body".to_string(), sha1(from_utf8(&base_request.body)?));
                }
            }
        }
        self.sign(queries)
    }

    pub fn sign(&self, queries: &HashMap<String, String>) -> anyhow::Result<String> {
        // let mut keys = queries
        //     .iter()
        //     .filter(|(k, v)| !k.eq("Signature".to_string().as_ref()))
        //     .collect::<Vec<&String>>();
        // let mut keys: Vec<String> = queries.keys().cloned().collect();
        let mut keys: Vec<String> = queries
            .keys()
            .filter(|k| !k.as_str().eq("Signature"))
            .map(|k| k.to_string())
            .collect();
        keys.sort();

        let mut buffer = String::new();
        for key in keys {
            if let Some(val) = queries.get(&key) {
                write!(buffer, "{}={}", key, val)?;
            }
        }
        buffer.push_str(&self.app_secret);

        Ok(md5(&buffer))
    }
}
