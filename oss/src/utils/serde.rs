pub mod option_time_rfc3339_serde {
    use std::time::SystemTime;

    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(option_time: &Option<SystemTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match option_time {
            Some(t) => serializer.serialize_some(
                &DateTime::<Utc>::from(*t).to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            ),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<SystemTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let option_string = Option::<String>::deserialize(deserializer)?;
        match option_string {
            Some(s) => Ok(Some(
                DateTime::parse_from_rfc3339(&s)
                    .map_err(serde::de::Error::custom)?
                    .into(),
            )),
            None => Ok(None),
        }
    }
}

#[allow(dead_code)]
pub mod option_time_rfc2822_serde {
    use std::time::SystemTime;

    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(option_time: &Option<SystemTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match option_time {
            Some(t) => serializer.serialize_some(
                &DateTime::<Utc>::from(*t)
                    .to_rfc2822()
                    .replace("+0000", "GMT"),
            ),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<SystemTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let option_string = Option::<String>::deserialize(deserializer)?;
        match option_string {
            Some(s) => Ok(Some(
                DateTime::parse_from_rfc2822(&s)
                    .map_err(serde::de::Error::custom)?
                    .into(),
            )),
            None => Ok(None),
        }
    }
}

pub mod bucket_properties_de {
    use serde::{Deserialize, Deserializer};

    use crate::api::bucket::BucketProperties;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<BucketProperties>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wrapper {
            #[serde(rename = "Bucket", default)]
            buckets: Vec<BucketProperties>,
        }

        Wrapper::deserialize(deserializer).map(|w| w.buckets)
    }
}

pub mod acl_grant_de {
    use std::str::FromStr;

    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: FromStr,
    {
        #[derive(Deserialize)]
        struct Wrapper {
            #[serde(rename = "Grant", default)]
            grant: Option<String>,
        }

        Wrapper::deserialize(deserializer).map(|w| w.grant.and_then(|s| s.parse().ok()))
    }
}

pub mod xml_escape_str_ser {
    use serde::Serializer;

    use crate::utils::escape_xml;

    pub fn serialize<S>(s: &str, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&escape_xml(s))
    }
}
