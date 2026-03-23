mod create_bucket;
mod delete_bucket;
mod get_bucket_acl;
mod get_bucket_info;
mod list_objects_v2;
mod put_bucket_acl;

use std::time::SystemTime;

use serde::{Deserialize, Serialize};

pub use self::get_bucket_acl::*;
pub use self::get_bucket_info::*;
pub use self::list_objects_v2::*;
pub use self::create_bucket::*;
pub use self::delete_bucket::*;
pub use self::put_bucket_acl::*;
use crate::utils::{acl_grant_de, option_time_rfc3339_serde};

#[derive(Debug, Default, Deserialize)]
pub struct BucketProperties {
    /// The name of the bucket.
    #[serde(rename = "Name")]
    pub name: Option<String>,

    /// The data center in which the bucket is located.
    #[serde(rename = "Location")]
    pub location: Option<String>,

    /// The time when the bucket was created. Format:
    /// yyyy-mm-ddThh:mm:ss.timezone.
    #[serde(rename = "CreationDate", with = "option_time_rfc3339_serde")]
    pub creation_date: Option<SystemTime>,

    /// The storage class of the bucket. Valid values:
    /// Standard, IA, Archive, ColdArchive, and DeepColdArchive.
    #[serde(rename = "StorageClass")]
    pub storage_class: Option<String>,

    /// The public endpoint used to access the bucket over the Internet.
    #[serde(rename = "ExtranetEndpoint")]
    pub extranet_endpoint: Option<String>,

    /// The internal endpoint that is used to access the bucket from ECS
    /// instances that reside in the same region as the bucket.
    #[serde(rename = "IntranetEndpoint")]
    pub intranet_endpoint: Option<String>,

    /// The region in which the bucket is located.
    #[serde(rename = "Region")]
    pub region: Option<String>,

    /// The ID of the resource group to which the bucket belongs.
    #[serde(rename = "ResourceGroupId")]
    pub resource_group_id: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct BucketInfo {
    /// Indicates whether access tracking is enabled for the bucket.
    #[serde(rename = "AccessMonitor")]
    pub access_monitor: Option<String>,

    /// The time when the bucket is created. The time is in UTC.
    #[serde(rename = "CreationDate", with = "option_time_rfc3339_serde", default)]
    pub creation_date: Option<SystemTime>,

    /// Indicates whether cross-region replication (CRR) is enabled for the
    /// bucket.
    #[serde(rename = "CrossRegionReplication")]
    pub cross_region_replication: Option<String>,

    /// The disaster recovery type of the bucket.
    #[serde(rename = "DataRedundancyType")]
    pub data_redundancy_type: Option<String>,

    /// The public endpoint that is used to access the bucket over the Internet.
    #[serde(rename = "ExtranetEndpoint")]
    pub extranet_endpoint: Option<String>,

    /// The internal endpoint that is used to access the bucket from Elastic
    #[serde(rename = "IntranetEndpoint")]
    pub intranet_endpoint: Option<String>,

    /// The region in which the bucket is located.
    #[serde(rename = "Location")]
    pub location: Option<String>,

    /// The name of the bucket.
    #[serde(rename = "Name")]
    pub name: Option<String>,

    /// The ID of the resource group to which the bucket belongs.
    #[serde(rename = "ResourceGroupId")]
    pub resource_group_id: Option<String>,

    /// The storage class of the bucket.
    #[serde(rename = "StorageClass")]
    pub storage_class: Option<String>,

    /// Indicates whether transfer acceleration is enabled for the bucket.
    #[serde(rename = "TransferAcceleration")]
    pub transfer_acceleration: Option<String>,

    /// The container that stores the information about the bucket owner.
    #[serde(rename = "Owner")]
    pub owner: Option<Owner>,

    /// The container that stores the access control list (ACL) information
    /// about the bucket.
    #[serde(rename = "AccessControlList", with = "acl_grant_de")]
    pub acl: Option<String>,

    /// The container that stores the server-side encryption method.
    #[serde(rename = "ServerSideEncryptionRule")]
    pub sse_rule: SSERule,

    /// The container that stores the logs.
    #[serde(rename = "BucketPolicy")]
    pub bucket_policy: BucketPolicy,

    /// Indicates whether versioning is enabled for the bucket.
    #[serde(rename = "Versioning")]
    pub versioning: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Owner {
    /// The ID of the bucket owner.
    #[serde(rename = "ID")]
    pub id: Option<String>,

    /// The name of the bucket owner.
    #[serde(rename = "DisplayName")]
    pub display_name: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SSERule {
    /// The customer master key (CMK) ID in use. A valid value is returned only
    /// if you set SSEAlgorithm to KMS and specify the CMK ID. In other
    /// cases, an empty value is returned.
    #[serde(rename = "KMSMasterKeyID")]
    pub kms_master_key_id: Option<String>,

    /// The server-side encryption method that is used by default.
    #[serde(rename = "SSEAlgorithm")]
    pub sse_algorithm: Option<String>,

    /// Object's encryption algorithm. If this element is not included in the
    /// response, it indicates that the object is using the AES256
    /// encryption algorithm. This option is only valid if the SSEAlgorithm
    /// value is KMS.
    #[serde(rename = "KMSDataEncryption")]
    pub kms_data_encryption: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BucketPolicy {
    /// The name of the bucket that stores the logs.
    #[serde(rename = "LogBucket")]
    pub log_bucket: Option<String>,

    /// The directory in which logs are stored.
    #[serde(rename = "LogPrefix")]
    pub log_prefix: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CommonPrefix {
    #[serde(rename = "Prefix")]
    pub prefix: String,
}
