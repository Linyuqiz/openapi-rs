use crate::openapi::client::OpenApiClient;
use crate::openapi::config::OpenApiConfig;
use md5::digest::typenum::op;

mod openapi;
mod utils;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("load .env file failed");
    let open_api_config = OpenApiConfig::new().load_from_env();
    let open_api_client = OpenApiClient::new(open_api_config);


    println!("{}", response);
}
