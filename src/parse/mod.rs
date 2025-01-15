use std::collections::HashMap;
use serde_json::Value;

pub mod text_parse;
pub mod set_parse;

pub mod if_parse;

pub mod foreach_parse;

pub mod variable_parse;



fn update_content(content: &mut HashMap<String, Value>, key:&str, new_value: Value) {
    if let Some(value) = content.get_mut(key) {
        *value = new_value;
    } else {
        content.insert(key.to_string(), new_value);
    }
}
