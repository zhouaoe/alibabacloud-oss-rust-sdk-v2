use alibabacloud_oss_sdk_rust_v2_api_model::{OssRequestModel, OssResultModel};
use serde::Deserialize;

use super::BucketInfo;
use crate::api::{RequestCommon, ResultCommon};
use crate::client::Client;
use crate::utils::{modify_request, update_content_md5};
use crate::{OperationInput, OperationOutput, DEFAULT_CONTENT_TYPE, HTTP_HEADER_CONTENT_TYPE};
use crate::client::BodyDataReader;


#[derive(Debug, Default, OssRequestModel)]
pub struct GetBucketInfoRequest {
    /// The name of the bucket containing the objects
    pub bucket: String,

    pub common: RequestCommon,
}

impl GetBucketInfoRequest {
    pub fn new(bucket: &str) -> Self {
        Self {
            bucket: bucket.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Deserialize, OssResultModel)]
pub struct GetBucketInfoResult {
    #[serde(rename = "Bucket")]
    pub bucket_info: BucketInfo,

    #[serde(skip)]
    pub common: ResultCommon,
}

impl Client {
    /// Retrieves information about a bucket.
    ///
    /// # Arguments
    ///
    /// * `request` - The `GetBucketInfoRequest` object containing the bucket
    ///   name.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `GetBucketInfoResult` if successful,
    /// or a boxed `dyn std::error::Error` if an error occurs.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use alibabacloud_oss_sdk_rust_v2::api::bucket::GetBucketInfoRequest;
    /// # use alibabacloud_oss_sdk_rust_v2::client::Client;
    /// # use alibabacloud_oss_sdk_rust_v2::config::Config;
    /// #
    /// # tokio_test::block_on(async {
    /// let client = Client::new(&Config::default());
    /// let request = GetBucketInfoRequest::new("my-bucket");
    ///
    /// match client.get_bucket_info(&request).await {
    ///     Ok(bucket_info) => {
    ///         // Handle bucket info
    ///     }
    ///     Err(err) => {
    ///         // Handle error
    ///     }
    /// }
    /// # })
    /// ```
    pub async fn get_bucket_info(
        &self,
        request: &GetBucketInfoRequest,
    ) -> Result<GetBucketInfoResult, Box<dyn std::error::Error + Send + Sync>> {
        let mut input = OperationInput {
            op_name: "GetBucketInfo".to_string(),
            method: http::Method::GET,
            bucket: Some(request.bucket.clone()),
            parameters: [("bucketInfo", "")]
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            headers: [(HTTP_HEADER_CONTENT_TYPE, DEFAULT_CONTENT_TYPE)]
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            ..Default::default()
        };

        modify_request(
            &mut input,
            request.header_map(),
            request.query_map(),
            vec![update_content_md5],
        )?;

        let mut output = self.invoke_operation(input, vec![]).await?;

        let body_bytes = output.get_all_data().await?;
        let body_data = String::from_utf8_lossy(&body_bytes).into_owned();
        let mut result: GetBucketInfoResult =
            quick_xml::de::from_str(&body_data)?;

        result.update_result(&output);

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::*;
    use crate::api::bucket::{CreateBucketRequest, DeleteBucketRequest};
    use crate::config::Config;
    use crate::credential::StaticCredentialsProvider;
    use crate::log::LogLevel;
    use crate::SignatureVersionType;
    use crate::test_utils::{load_test_config, TestConfig, generate_unique_bucket_name};

    #[tokio::test]
    #[serial_test::serial]
    async fn test_get_bucket_operation() {
        let config = match load_test_config() {
            Some(cfg) => cfg,
            None => {
                eprintln!("Test configuration not found. Skipping test.");
                return;
            }
        };

        let client = Client::new(
            &Config::default()
                .with_region(&config.region)
                .with_credentials_provider(Rc::new(StaticCredentialsProvider::new(
                    &config.access_key_id,
                    &config.access_key_secret,
                    &[],
                )))
                .with_signature_version(SignatureVersionType::V4)
                .with_log_level(LogLevel::Debug),
        );

        // Generate a unique bucket name for this test
        let bucket_name = generate_unique_bucket_name("get-bucket-info");

        // Create the bucket first
        let create_request = CreateBucketRequest {
            bucket: bucket_name.clone(),
            ..Default::default()
        };
        
        match client.create_bucket(&create_request).await {
            Ok(_) => println!("Bucket created: {}", bucket_name),
            Err(err) => panic!("Failed to create bucket: {:?}", err),
        };

        // Perform the test
        match client.get_bucket_info(&GetBucketInfoRequest::new(&bucket_name)).await {
            Ok(output) => {
                println!("{:?}", output);
                // Clean up: delete the bucket
                let delete_request = DeleteBucketRequest {
                    bucket: bucket_name.clone(),
                    ..Default::default()
                };
                match client.delete_bucket(&delete_request).await {
                    Ok(_) => println!("Bucket deleted: {}", bucket_name),
                    Err(err) => eprintln!("Failed to delete bucket: {:?}", err),
                }
            },
            Err(err) => {
                // Even if the test fails, try to clean up
                let delete_request = DeleteBucketRequest {
                    bucket: bucket_name.clone(),
                    ..Default::default()
                };
                match client.delete_bucket(&delete_request).await {
                    Ok(_) => println!("Bucket deleted: {}", bucket_name),
                    Err(err) => eprintln!("Failed to delete bucket: {:?}", err),
                }
                panic!("Invoke operation failed: {:?}", err);
            }
        }
    }
}
