use std::collections::HashMap;
use serde_json::{Number, Value};
use crate::parse::variable_parse;
use crate::token::token_parse::Tokenizer;

pub fn set_parse(token :&Tokenizer, content: &mut HashMap<String, Value>) -> Option<String>{

    if let Tokenizer::Set { key,value } = token {
        // println!("key:{} vlaue:{}",key,value);
        let k = variable_parse::extract_variable(&key);
        if let Some(key) = k{
            if let Ok(number) = value.trim_matches('"').parse::<isize>() {
                // println!("Parsed as number: key:{} value:{}", key, number);
                if let Some(value) = content.get_mut(&key) {
                    *value = Value::Number(Number::from(number));
                }else{
                    content.insert(key, Value::Number(Number::from(number)));
                }
            } else {
                // println!("Parsed as string: key:{} value:{}", key, value);
                if let Some(value) = content.get_mut(&key) {
                    *value = Value::String(value.to_string());
                }else{
                    content.insert(key, Value::String(value.to_string()));
                }
            }
        }

    }

    None
}