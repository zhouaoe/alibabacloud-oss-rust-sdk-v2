# Alibaba Cloud OSS SDK for Rust V2

Rust SDK for [Alibaba Cloud Object Storage Service (OSS)](https://www.alibabacloud.com/product/oss) V2.

## Table of Contents

- [Overview](#overview)
- [Quick Start](#quick-start)
- [Installation](#installation)
- [Configuration](#configuration)
- [Examples](#examples)
- [API Reference](#api-reference)
- [Architecture](#architecture)
- [Testing](#testing)
- [Development](#development)
- [Contributing](#contributing)
- [Troubleshooting](#troubleshooting)
- [Additional Resources](#additional-resources)

## Overview

This SDK provides a comprehensive set of APIs for interacting with Alibaba Cloud Object Storage Service (OSS) in Rust. The APIs are organized into three main categories:

- **Bucket APIs**: Operations related to bucket management
- **Object APIs**: Operations related to object management
- **Service APIs**: Operations related to service-level management

### Key Features

- **Type-Safe API Design**: Strongly-typed request and response structures
- **Automatic Serialization**: Header/query parameter handling via macros
- **Comprehensive Testing**: Extensive integration tests with automatic resource cleanup
- **Flexible Configuration**: Support for various authentication methods
- **Robust Error Handling**: Custom error types and retry mechanisms
- **Logging Support**: Configurable logging levels and outputs

## Quick Start

### 1. Setup Environment Variables

```bash
export ACCESS_KEY_ID="your-access-key-id"
export ACCESS_KEY_SECRET="your-access-key-secret"
export OSS_REGION="cn-hangzhou"
export OSS_BUCKET="your-bucket-name"
```

### 2. Basic Example

```rust
use alibabacloud_oss_sdk_rust_v2::{
    api::object::{GetObjectRequest, PutObjectRequest},
    client::Client,
    config::Config,
    credential::providers::StaticCredentialsProvider,
    BodyContent,
};
use std::rc::Rc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure the client
    let config = Config::default()
        .with_region(&std::env::var("OSS_REGION")?)
        .with_credentials_provider(Rc::new(StaticCredentialsProvider::new(
            &std::env::var("ACCESS_KEY_ID")?,
            &std::env::var("ACCESS_KEY_SECRET")?,
            &[],
        )))
        .with_signature_version(alibabacloud_oss_sdk_rust_v2::SignatureVersionType::V4);

    let client = Client::new(&config);

    // Upload an object
    let put_request = PutObjectRequest {
        bucket: std::env::var("OSS_BUCKET")?,
        key: "hello.txt".to_string(),
        body: Some(BodyContent::from_text("Hello OSS!".to_string(), None)),
        ..Default::default()
    };

    match client.put_object(put_request).await {
        Ok(result) => println!("Upload successful!"),
        Err(e) => eprintln!("Upload failed: {}", e),
    }

    // Download the object
    let get_request = GetObjectRequest {
        bucket: std::env::var("OSS_BUCKET")?,
        key: "hello.txt".to_string(),
        ..Default::default()
    };

    match client.get_object(get_request).await {
        Ok(mut result) => {
            use alibabacloud_oss_sdk_rust_v2::client::BodyDataReader;
            let data = result.get_all_data().await?;
            println!("Content: {}", String::from_utf8_lossy(&data));
        }
        Err(e) => eprintln!("Download failed: {}", e),
    }

    Ok(())
}
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
alibabacloud-oss-sdk-rust-v2 = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

Then run:

```bash
cargo build
```

## Configuration

### Authentication Methods

#### Static Credentials

```rust
let config = Config::default()
    .with_region("cn-hangzhou")
    .with_credentials_provider(Rc::new(StaticCredentialsProvider::new(
        "access_key_id",
        "access_key_secret",
        &[],
    )))
    .with_signature_version(SignatureVersionType::V4);
```

#### Environment Variables

```rust
let access_key_id = std::env::var("ACCESS_KEY_ID")?;
let access_key_secret = std::env::var("ACCESS_KEY_SECRET")?;
```

#### ECS Role (for applications running on ECS)

```rust
use alibabacloud_oss_sdk_rust_v2::credential::providers::EcsRoleCredentialsProvider;

let config = Config::default()
    .with_region("cn-hangzhou")
    .with_credentials_provider(Rc::new(EcsRoleCredentialsProvider::new()));
```

### Advanced Configuration

```rust
let config = Config::default()
    .with_region("cn-hangzhou")
    .with_credentials_provider(credentials_provider)
    .with_signature_version(SignatureVersionType::V4)
    .with_retry_max_attempts(3)
    .with_retry_initial_interval(1.0)
    .with_retry_max_interval(60.0)
    .with_log_level(LogLevel::Debug);
```

## Examples

We provide comprehensive examples in the `oss/examples` directory:

### Available Examples

1. **Quickstart** (`01_quickstart.rs`)
   - Basic client setup
   - Upload and download objects
   - Get bucket information
   - Run: `cargo run --example 01_quickstart`

2. **File Upload** (`02_file_upload.rs`)
   - Upload text content
   - Upload binary data
   - Set custom metadata
   - Multipart upload for large files
   - Run: `cargo run --example 02_file_upload`

3. **Bucket Management** (`03_bucket_management.rs`)
   - Get bucket information
   - Manage bucket ACL
   - List objects in bucket
   - Create/delete buckets
   - Run: `cargo run --example 03_bucket_management`

4. **Error Handling** (`04_error_handling.rs`)
   - Handle common errors (404, permission denied, etc.)
   - Custom error types
   - Retry strategies
   - Graceful error recovery
   - Run: `cargo run --example 04_error_handling`

### Running Examples

```bash
# Navigate to project root
cd /path/to/aliyun-oss-sdk-rust-v2

# Set environment variables
export ACCESS_KEY_ID="your-access-key-id"
export ACCESS_KEY_SECRET="your-access-key-secret"
export OSS_REGION="cn-hangzhou"
export OSS_BUCKET="your-bucket-name"

# Run specific example
cargo run --example 01_quickstart
cargo run --example 02_file_upload
cargo run --example 03_bucket_management
cargo run --example 04_error_handling
```

## API Reference

### Bucket APIs

- `create_bucket` - Create a new bucket
- `delete_bucket` - Delete an existing bucket
- `get_bucket_acl` - Get bucket access control list
- `get_bucket_info` - Get detailed bucket information
- `list_objects_v2` - List objects in a bucket (V2 API)
- `put_bucket_acl` - Set bucket access control list

### Object APIs

#### Basic Operations
- `get_object` - Retrieve an object
- `put_object` - Upload an object
- `delete_object` - Delete an object
- `copy_object` - Copy an object
- `head_object` - Get object metadata without content

#### Access Control
- `get_object_acl` - Get object access control list
- `put_object_acl` - Set object access control list
- `get_object_meta` - Get object metadata

#### Batch Operations
- `delete_multiple_objects` - Delete multiple objects in one request

#### Multipart Upload
- `initiate_multipart_upload` - Start multipart upload
- `upload_part` - Upload a part
- `complete_multipart_upload` - Complete multipart upload
- `abort_multipart_upload` - Abort multipart upload
- `list_multipart_uploads` - List active multipart uploads
- `list_parts` - List uploaded parts
- `upload_part_copy` - Copy part from another object

### Service APIs

- `list_buckets` - List all buckets owned by the account

## Architecture

### Project Structure

```
aliyun-oss-sdk-rust-v2/
├── oss/                          # Main SDK implementation
│   ├── src/
│   │   ├── api/                  # API definitions
│   │   │   ├── bucket/          # Bucket operations
│   │   │   ├── object/          # Object operations
│   │   │   ├── service/         # Service operations
│   │   │   └── mod.rs
│   │   ├── client/              # Client implementation
│   │   ├── credential/          # Authentication providers
│   │   ├── retry/               # Retry mechanisms
│   │   ├── signer/              # Request signing
│   │   ├── transport/           # HTTP transport
│   │   ├── types/               # Type definitions
│   │   ├── utils/               # Utility functions
│   │   └── lib.rs
│   └── examples/                # Code examples
├── api_model/                   # API model macros
└── samples/                     # Additional samples
```

### Core Components

#### Request/Response Models

The SDK uses procedural macros to generate request and response models:

```rust
// Request structure
#[derive(Debug, OssRequestModel)]
pub struct GetObjectRequest {
    pub bucket: String,
    pub key: String,
    #[field(type = "header", rename = "If-Match")]
    pub if_match: Option<String>,
    #[field(type = "query", rename = "versionId")]
    pub version_id: Option<String>,
    pub common: RequestCommon,
}

// Response structure
#[derive(Debug, OssResultModel)]
pub struct GetObjectResult {
    #[field(type = "header", rename = "Content-Length")]
    pub content_length: Option<u64>,
    #[field(type = "header", rename = "ETag")]
    pub etag: Option<String>,
    pub common: ResultCommon,
}
```

#### Logging System

```rust
use alibabacloud_oss_sdk_rust_v2::log::{Logger, LogLevel, LogOutput};

// Configure logging
let logger = Logger::new()
    .with_level(LogLevel::Debug)
    .with_output(LogOutput::Stdout);
```

## Testing

### Run Unit Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test module
cargo test --lib client
```

### Generate Coverage Report

```bash
# Install cargo-llvm-cov
cargo install cargo-llvm-cov

# Generate HTML report
cargo llvm-cov --html

# Generate lcov report
cargo llvm-cov --lcov --output-path lcov.info
```

### Integration Tests with OSS Service

1. Obtain your AK/SK (AccessKey ID and AccessKey Secret)
   - **Recommended**: Create a RAM user with `AliyunOSSFullAccess` policy
   - **Alternative**: Use primary account AccessKey (not recommended for production)

2. Create an OSS bucket in the [OSS Console](https://oss.console.aliyun.com/bucket)

3. Create `test_config.json` in the project root:

```json
{
  "region": "cn-hangzhou",
  "bucket": "your-bucket-name",
  "version_bucket": "your-versioned-bucket-name",
  "object": "your-test-object-name",
  "access_key_id": "your-access-key-id",
  "access_key_secret": "your-access-key-secret"
}
```

4. Run tests:

```bash
cargo test
```

## Development

### Prerequisites

Install [pre-commit](https://pre-commit.com) for code quality:

```bash
pip install pre-commit
pre-commit install
```

### Build from Source

```bash
git clone https://github.com/aliyun/aliyun-oss-rust-sdk.git
cd aliyun-oss-rust-sdk
cargo build --release
```

### Adding a New API

1. Create `{api_name}.rs` in `oss/src/api/{service|bucket|object}/`
2. Define `ApiNameRequest` and `ApiNameResult` structs
3. Implement `api_name()` method for `Client`
4. Update corresponding `mod.rs`
5. Add tests in `oss/tests/`

Example:

```rust
// oss/src/api/object/my_api.rs
#[derive(Debug, OssRequestModel)]
pub struct MyApiRequest {
    pub bucket: String,
    pub key: String,
    #[field(type = "query")]
    pub param: Option<String>,
    pub common: RequestCommon,
}

#[derive(Debug, OssResultModel)]
pub struct MyApiResult {
    #[field(type = "header")]
    pub custom_header: Option<String>,
    pub common: ResultCommon,
}

impl Client {
    pub async fn my_api(&self, request: &MyApiRequest) -> Result<MyApiResult, Box<dyn std::error::Error + Send + Sync>> {
        // Implementation
    }
}
```

## Contributing

We welcome contributions! Here's how you can help:

### Ways to Contribute

- Report bugs and issues
- Suggest new features
- Improve documentation
- Submit pull requests
- Add new API implementations
- Enhance test coverage

### Guidelines

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/your-feature`
3. **Make your changes**
4. **Run tests**: Ensure all tests pass
5. **Run pre-commit hooks**: `pre-commit run --all-files`
6. **Commit with clear messages**
7. **Submit a pull request**

### Good First Issues

Look for issues labeled "good first issue" or "help wanted" to get started.

## Troubleshooting

### Common Issues

#### Authentication Failed

```
Error: InvalidAccessKeyId
```

**Solution**:
- Verify `ACCESS_KEY_ID` and `ACCESS_KEY_SECRET` are correct
- Ensure the AccessKey is active in RAM console
- Check for extra spaces in environment variables

#### Bucket Not Found

```
Error: NoSuchBucket
```

**Solution**:
- Verify the bucket exists in the specified region
- Check `OSS_BUCKET` environment variable
- Ensure region matches bucket location

#### Permission Denied

```
Error: AccessDenied
```

**Solution**:
- Verify RAM user has necessary OSS permissions
- Check bucket ACL and policy settings
- Use `AliyunOSSFullAccess` policy for testing

#### Network Errors

```
Error: Connection timeout
```

**Solution**:
- Check network connection
- Verify region endpoint is accessible
- Implement retry logic (see error handling example)
- Adjust timeout settings in configuration

### Getting Help

1. Check the [FAQ](https://www.alibabacloud.com/help/en/oss/faq)
2. Review existing GitHub issues
3. Create a new issue with detailed information
4. Contact Alibaba Cloud support

## Additional Resources

### Documentation

- [Official OSS Documentation](https://www.alibabacloud.com/help/en/oss)
- [API Reference](https://www.alibabacloud.com/help/en/oss/api-reference)
- [SDK Source Code](https://github.com/aliyun/aliyun-oss-rust-sdk)
- [Pricing Calculator](https://www.alibabacloud.com/pricing-calculator)

### Related Projects

- [Alibaba Cloud SDK for Python](https://github.com/aliyun/aliyun-openapi-python-sdk)
- [Alibaba Cloud SDK for Java](https://github.com/aliyun/aliyun-openapi-java-sdk)
- [Alibaba Cloud SDK for Go](https://github.com/aliyun/aliyun-oss-go-sdk)

### Security Best Practices

- Never hardcode credentials in source code
- Use environment variables or secure secret management
- Apply principle of least privilege for RAM policies
- Use HTTPS for all OSS operations
- Rotate AccessKeys regularly
- Enable OSS access logging for audit

---

**License**: Apache License 2.0

**Support**: For questions and issues, please open a GitHub issue or contact Alibaba Cloud support.
