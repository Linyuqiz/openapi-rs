# openapi-rs

[![Crates.io](https://img.shields.io/crates/v/openapi-rs.svg)](https://crates.io/crates/openapi-rs)
[![License](https://img.shields.io/crates/l/openapi-rs.svg)](https://github.com/Linyuqiz/openapi-rs/blob/main/LICENSE)
[![Rust](https://img.shields.io/badge/rust-2024-blue.svg)](https://www.rust-lang.org)

A type-safe Rust client library for building, signing, and sending API requests with minimal boilerplate.

## Features

- **Type-safe API interactions**: Leverage Rust's type system for compile-time safety
- **Async/await support**: Built on Tokio for efficient asynchronous operations
- **Environment-based configuration**: Easily load configuration from environment variables
- **Automatic request signing**: Handles authentication and request signing automatically
- **Multiple endpoint support**: Support for different endpoint types (API, Cloud, HPC, Sync)
- **Extensible architecture**: Easily add new API endpoints with minimal code

## Installation

Add the dependency to your `Cargo.toml`:

```bash
cargo add openapi-rs
```

Or manually add to your `Cargo.toml`:

```toml
[dependencies]
openapi-rs = "0.1.2"
```

## Quick Start

```rust
use openapi_rs::api::v1::job::any_zone_list::AnyZoneListRequest;
use openapi_rs::common::client::OpenApiClient;
use openapi_rs::common::config::OpenApiConfig;
use openapi_rs::common::define::HttpBuilder;
use tracing::info;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Load environment variables
    dotenvy::dotenv()?;
    
    // Create and configure the client
    let config = OpenApiConfig::new().load_from_env()?;
    let mut client = OpenApiClient::new(config);

    // Build and send the request
    let http_fn = AnyZoneListRequest::new().builder();
    let response = client.send(http_fn).await?;
    info!("response: {:#?}", response);

    Ok(())
}
```

## Configuration

### Environment Variables

The library uses the following environment variables for configuration:

```
OpenApiAppKey=your_app_key
OpenApiAppSecret=your_app_secret
OpenApiEndpoint=https://api.example.com
OpenApiCloudEndpoint=https://cloud.example.com
OpenApiHpcEndpoint=https://hpc.example.com
OpenApiSyncEndpoint=https://sync.example.com
OpenApiUserId=your_user_id
OpenApiZone=your_zone
XYsVersion=your_version
```

You can create a `.env` file in your project root with these variables or set them in your environment.

### Manual Configuration

You can also configure the client programmatically:

```rust
let config = OpenApiConfig::new()
    .with_app_key("your_app_key".to_string())
    .with_app_secret("your_app_secret".to_string())
    .with_endpoint("https://api.example.com".to_string())
    .with_cloud_endpoint("https://cloud.example.com".to_string())
    .with_hpc_endpoint("https://hpc.example.com".to_string())
    .with_user_id("your_user_id".to_string())
    .with_zone("your_zone".to_string());
```

## API Documentation

### Implementation Status

**Note**: Currently, only a subset of API interfaces has been implemented. Additional interfaces will be implemented progressively based on specific requirements. If you need an interface that hasn't been implemented yet, feel free to open an issue or submit a pull request. Alternatively, you can refer to the "Creating Custom API Requests" section below to implement your own interfaces.

### Available APIs

The library currently supports the following API categories:

- **Job API**: Manage and monitor jobs
  - `AnyZoneListRequest`: List available zones
  - `ApiJobGetRequest`: Get job details
  - `ApiJobListRequest`: List jobs
  - `AdminJobGetRequest`: Admin-level job operations

- **Storage API**: Manage storage resources

- **Merch API**: Merchandise-related operations

- **Sync API**: Synchronization operations

### Creating Custom API Requests

You can create custom API requests by implementing the `HttpBuilder` trait:

```rust
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct MyCustomRequest {
    // Your request fields here
}

impl HttpBuilder for MyCustomRequest {
    type Response = BaseResponse<MyCustomResponse>;
    fn builder(self) -> HttpFn<Self::Response> {
        Box::new(move || {
            let request_fn: RequestFn = Box::new(|| BaseRequest {
                method: Method::GET,
                uri: "/api/my-endpoint".to_string(),
                // Configure other request parameters
                ..Default::default()
            });
            let response_fn: AsyncResponseFn<Self::Response> =
                Box::new(|response: Response| Box::pin(async move { Ok(response.json().await?) }));
            (request_fn, response_fn)
        })
    }
}
```

## Examples

Check the `examples` directory for complete working examples:

- **Zone List Example**: Demonstrates how to list available zones

To run an example:

```bash
cd examples/zone-list
cargo run
```

## Error Handling

The library uses `anyhow` for error handling, providing detailed error information. All API methods return `anyhow::Result<T>` which can be handled using standard Rust error handling patterns:

```rust
match client.send(http_fn).await {
    Ok(response) => {
        // Handle successful response
        println!("Success: {:?}", response);
    },
    Err(e) => {
        // Handle error
        eprintln!("Error: {}", e);
    }
}
```

## Contributing

Contributions are welcome! Here's how you can contribute:

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Commit your changes: `git commit -am 'Add my feature'`
4. Push to the branch: `git push origin feature/my-feature`
5. Submit a pull request

### Development Setup

```bash
# Clone the repository
git clone https://github.com/Linyuqiz/openapi-rs.git
cd openapi-rs

# Copy example environment file and configure it
cp .env.example .env
# Edit .env with your API credentials

# Run tests
cargo test
```

## License

This project is licensed under either of:

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.