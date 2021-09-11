use random_string::generate;
use serde_json::{json, Value};

pub use create_connector::*;
pub use create_connector_relation_instance_with_properties::*;
pub use create_random_entity_instance::*;

pub mod create_connector;
pub mod create_connector_relation_instance_with_properties;
pub mod create_random_entity_instance;

const CHARSET_LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn r_string() -> String {
    generate(10, CHARSET_LETTERS).to_string()
}

pub fn r_string_255() -> String {
    generate(255, CHARSET_LETTERS).to_string()
}

pub fn r_string_1000() -> String {
    generate(1000, CHARSET_LETTERS).to_string()
}

pub fn r_json_string() -> Value {
    json!(r_string())
}
