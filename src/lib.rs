use crate::token::token_parse;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::ops::Add;

pub mod expression;

pub mod token;

pub mod parse;


pub fn render_default(template:&str)-> Result<String, String> {
    let mut content:HashMap<String, Value> = HashMap::new();
    render(template, &mut content)
}

pub fn render(template: &str, content: &mut HashMap<String, Value>) -> Result<String, String> {
    let tokens_result = token_parse::get_tokens(template);
    match tokens_result {
        Ok(tokens) => {
            let output = token_parse::parse_tokens(&tokens, content);
            if let Some(output) = output {
                return Ok(output);
            }
            Ok(String::new())
        }
        Err(error) => Err(error.to_string()),
    }
}

pub fn render_from_path(path: &str, content: &mut HashMap<String, Value>, ) -> Result<String, String> {
    match read_file(path) {
        Ok(template) => match render(template.as_str(), content) {
            Ok(content) => Ok(content),
            Err(error) => Err(error),
        },
        Err(error) => Err(error),
    }
}

pub fn render_from_object<T: Serialize>(template: &str, obj: &T) -> Result<String, String> {
    let mut context_result = object_to_hashmap(obj);
    match context_result {
        Ok(mut context_map) => match render(template, &mut context_map) {
            Ok(content) => Ok(content),
            Err(error) => Err(error),
        },
        Err(error) => Err(error),
    }
}

pub fn object_to_hashmap<T: Serialize>(obj: &T) -> Result<HashMap<String, Value>, String> {
    match serde_json::to_value(obj) {
        Ok(Value::Object(map)) => Ok(map.into_iter().collect()),
        Ok(_) => Err("The serialized object is not a JSON object.".to_string()),
        Err(err) => Err(format!("Error serializing object: {}", err)),
    }
}

pub fn read_file(file_path: &str) -> Result<String, String> {
    std::fs::read_to_string(file_path)
        .map_err(|err| format!("Failed to read file '{}': {}", file_path, err))
}
