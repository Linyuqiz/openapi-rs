use std::env;

#[derive(Default, Debug)]
pub struct OpenApiConfig {
    pub app_key: String,
    pub app_secret: String,
    pub endpoint: String,
    pub cloud_endpoint: String,
    pub hpc_endpoint: String,
    pub sync_endpoint: String,
    pub user_id: String,
    pub zone: String,
}

#[derive(Debug, Default)]
pub enum EndpointType {
    #[default]
    Api,
    Cloud,
    Hpc,
    Sync,
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

    pub fn with_cloud_endpoint(mut self, cloud_endpoint: String) -> Self {
        self.cloud_endpoint = cloud_endpoint;
        self
    }

    pub fn with_hpc_endpoint(mut self, hpc_endpoint: String) -> Self {
        self.hpc_endpoint = hpc_endpoint;
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
            cloud_endpoint: env::var("OpenApiCloudEndpoint")?,
            hpc_endpoint: env::var("OpenApiHpcEndpoint")?,
            sync_endpoint: env::var("OpenApiSyncEndpoint")?,
            user_id: env::var("OpenApiUserId")?,
            zone: env::var("OpenApiZone")?,
        })
    }
}
