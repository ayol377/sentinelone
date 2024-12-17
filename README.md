# SentinelOne API Wrapper
========================

This crate provides a safe and easy way to interact with the SentinelOne API.

It supports all the endpoints as documented in the [SentinelOne API documentation](https://docs.sentinelone.com/API).

## Table of Contents

* [Usage](#usage)
* [Examples](#examples)
* [API](#api)
* [Error Handling](#error-handling)
* [Release History](#release-history)

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
sentinelone = "0.1.0"
```

## Examples

Here's a basic example of using the SentinelOne API client:

```rust
use sentinelone::{ClientConfig, XdrClient, Result};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize client configuration
    let config = ClientConfig::new(
        "your-api-token-here",
        "https://your-sentinelone-instance.com"
    )
    .with_timeout(Duration::from_secs(60));

    // Create XDR client
    let client = XdrClient::new(config)?;

    // Get unresolved threats
    let threats = client.get_threats(sentinelone::xdr::GetThreatsParams {
        limit: Some(10),
        skip: Some(0),
        resolved: Some(false),
    }).await?;

    println!("Found threats: {:?}", threats);
    Ok(())
}
```

## API

The crate provides several main components:

### ClientConfig

Configuration for the API client:
- `api_token`: Your SentinelOne API token
- `base_url`: The base URL of your SentinelOne instance
- `timeout`: Request timeout (defaults to 30 seconds)

### XdrClient

Main client for XDR-related operations:
- `get_threats()`: Retrieve threat information
- `get_agents()`: Retrieve agent information

### Error Handling

The crate uses a custom `Result` type that can return the following errors:
- `RequestError`: HTTP request failures
- `ConfigError`: Invalid API configuration
- `ApiError`: API response errors

## Examples

Check out the `examples/` directory for more detailed examples:
- `basic_usage.rs`: Basic usage of the API client
