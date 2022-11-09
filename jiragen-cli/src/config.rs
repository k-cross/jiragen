use jiragen::Config;
use std::fs::read_to_string;

/// Reads the contents of the given file path and returns a JiraGen Config object
pub fn read_config_file(path: &str) -> Config {
    let config_str = &read_to_string(path).unwrap();
    let json = serde_json::from_str(config_str).unwrap();

    json
}
