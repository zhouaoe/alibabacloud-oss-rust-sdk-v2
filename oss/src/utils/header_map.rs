use std::collections::HashMap;

use http::HeaderMap;

pub fn header_map_to_hash_map(headers: &HeaderMap) -> HashMap<String, String> {
    let mut map = HashMap::new();

    for (key, value) in headers.iter() {
        let key_string = key.as_str().to_string();
        let value_string = value.to_str().unwrap_or("").to_string();

        map.insert(key_string, value_string);
    }

    map
}
