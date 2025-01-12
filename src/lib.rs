use std::collections::HashMap;
use std::f32::consts::E;
use std::ops::Add;
use serde::Serialize;
use serde_json::{Value};
use serde_json::Value::String as JsonString;
use crate::token::token_parse;

pub mod expression;

pub mod token;

pub mod parse;



pub fn parse_template(template: &str,content: &mut HashMap<String, Value>) -> Result<String, String> {
    let tokens_result = token_parse::get_tokens(template);
    match tokens_result {
        Ok(tokens) => {
            let output = token_parse::parse_tokens(&tokens,content);
            if let Some(output) = output {
                return Ok(output);
            }
            Ok(String::new())
        }
        Err(error) => {
            Err(error.to_string())
        }
    }
}

pub fn parse_path_template(path:&str,content:&mut HashMap<String, Value>) -> Result<String, String>  {
    match read_file(path) {
        Ok(template) => {

            match parse_template(template.as_str(),content) {
                Ok(content) => Ok(content),
                Err(error) => Err(error)
            }

        },
        Err(error) => {
            Err(error)
        }
    }
}


pub fn parse_template_object<T: Serialize>(template:&str,obj: &T)-> Result<String, String> {
    let mut context_result = object_to_hashmap(obj);
    match context_result {
        Ok(mut context_map) => {

            match  parse_template(template,&mut context_map) {
                Ok(content) => Ok(content),
                Err(error) => Err(error)
            }

        },
        Err(error) => Err(error)
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


