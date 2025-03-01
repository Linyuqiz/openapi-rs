use std::env;

#[derive(Default, Debug)]
pub struct OpenApiConfig {
    pub app_key: String,
    pub app_secret: String,
    pub endpoint: String,
    pub user_id: String,
    pub zone: String,
}

impl OpenApiConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_app_key(mut self, app_key: String) -> Self {
        self.app_key = app_key;
        self
    }

    pub fn with_app_secret(mut self, app_secret: String) -> Self {
        self.app_secret = app_secret;
        self
    }

    pub fn with_endpoint(mut self, endpoint: String) -> Self {
        self.endpoint = endpoint;
        self
    }

    pub fn with_user_id(mut self, user_id: String) -> Self {
        self.user_id = user_id;
        self
    }

    pub fn with_zone(mut self, zone: String) -> Self {
        self.zone = zone;
        self
    }

    pub fn load_from_env(&mut self) -> anyhow::Result<Self> {
        Ok(Self {
            app_key: env::var("OpenApiAppKey")?,
            app_secret: env::var("OpenApiAppSecret")?,
            endpoint: env::var("OpenApiEndpoint")?,
            user_id: env::var("OpenApiUserId")?,
            zone: env::var("OpenApiZone")?,
        })
    }
}
