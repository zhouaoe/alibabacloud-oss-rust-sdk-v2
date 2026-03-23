use std::fs;
use std::path::Path;

#[derive(serde::Deserialize, Debug)]
pub struct TestConfig {
    pub region: String,
    pub bucket: String,
    pub object: String,
    pub access_key_id: String,
    pub access_key_secret: String,
    #[serde(rename = "version_bucket", default)]
    pub version_bucket: Option<String>,
}

/// Function to load test configuration from file
pub fn load_test_config() -> Option<TestConfig> {
    let config_paths = [
        "./test_config.json",          // Current directory (for IDE runs)
        "../test_config.json",         // Parent directory
        "../../test_config.json",      // Two levels up
        "../../../test_config.json",   // Three levels up (original)
        "../../../../test_config.json", // Four levels up (original)
    ];

    for path in &config_paths {
        if Path::new(path).exists() {
            match fs::read_to_string(path) {
                Ok(content) => match serde_json::from_str::<TestConfig>(&content) {
                    Ok(config) => return Some(config),
                    Err(_) => continue,
                },
                Err(_) => continue,
            }
        }
    }
    None
}

/// Helper function to generate unique test object names
pub fn generate_unique_object_name(base_name: &str) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();
    format!("{}_{}_{}", "test-oss-object", base_name, timestamp)
}

/// Helper function to generate unique bucket names for tests
/// Bucket names must follow specific naming rules and are typically globally unique
pub fn generate_unique_bucket_name(base_name: &str) -> String {
    let unique_name = generate_unique_object_name(base_name);
    // Bucket names typically use hyphens instead of underscores
    unique_name.replace('_', "-")
}