use std::time::SystemTime;

/// Represents the credentials.
#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Credentials {
    // TODO remove serde macro
    #[serde(rename = "AccessKeyId")]
    pub access_key_id: String,
    #[serde(rename = "AccessKeySecret")]
    pub access_key_secret: String,
    #[serde(rename = "SecurityToken")]
    pub security_token: String,
    #[serde(rename = "Expiration")]
    pub expires: Option<SystemTime>,
}

#[allow(clippy::derivable_impls)]
impl Default for Credentials {
    fn default() -> Self {
        Credentials {
            access_key_id: String::new(),
            access_key_secret: String::new(),
            security_token: String::new(),
            expires: None,
        }
    }
}

impl Credentials {
    /// Checks if the credentials have expired.
    pub fn expired(&self) -> bool {
        match self.expires {
            Some(expire_time) => expire_time <= SystemTime::now(),
            None => false,
        }
    }

    /// Checks if both AccessKeyID and AccessKeySecret are provided.
    pub fn has_keys(&self) -> bool {
        !self.access_key_id.is_empty() && !self.access_key_secret.is_empty()
    }
}

#[cfg(test)]
mod tests_credentials {
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_none_expires_credentials() {
        let credentials = Credentials {
            expires: None,
            ..Default::default()
        };
        assert!(!credentials.expired());
    }

    #[test]
    fn test_expired_credentials() {
        let credentials = Credentials {
            expires: Some(SystemTime::now() - Duration::new(3600, 0)),
            ..Default::default()
        };

        assert!(credentials.expired());
    }

    #[test]
    fn test_not_expired_credentials() {
        let credentials = Credentials {
            expires: Some(SystemTime::now() + Duration::new(3600, 0)),
            ..Default::default()
        };

        assert!(!credentials.expired());
    }

    #[test]
    fn test_has_keys_with_keys() {
        let credentials = Credentials {
            access_key_id: "access_key_id".to_string(),
            access_key_secret: "access_key_secret".to_string(),
            ..Default::default()
        };

        assert!(credentials.has_keys());
    }

    #[test]
    fn test_serde_credentials() {
        let credentials = Credentials {
            access_key_id: "access_key_id".to_string(),
            access_key_secret: "access_key_secret".to_string(),
            security_token: "security_token".to_string(),
            expires: Some(SystemTime::now() + Duration::new(3600, 0)),
        };
        let serialized_credentials = serde_json::to_string(&credentials).unwrap();
        let deserialized_credentials = serde_json::from_str(&serialized_credentials).unwrap();
        assert_eq!(credentials, deserialized_credentials);
    }
}
