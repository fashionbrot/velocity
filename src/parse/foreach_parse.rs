use std::collections::HashMap;
use serde_json::Value;
use crate::parse::{text_parse, variable_parse};
use crate::token::token_parse::{parse_token, Tokenizer};

pub fn foreach_parse(token:&Tokenizer, context:&mut HashMap<String, Value>) -> Option<std::string::String> {

    if let Tokenizer::Foreach { element,collection,children } = token {
        let mut output = String::new();

        let mut element_key = element.to_string();
        if let Some(key) = variable_parse::extract_variable(&element_key) {
            element_key  = key.to_string();
        }
        let mut collection_key = collection.to_string();
        if let Some(key) = variable_parse::extract_variable(&collection_key) {
            collection_key = key.to_string();
        }


        log::debug!("parse_foreach key:{:?} element:{} collection:{}",element_key,element,collection);

        // 从 context 中获取集合对象
        if let Some(Value::Array(list)) = context.get(&collection_key) {
            // 将集合对象更新到 context 中
            // context.insert(key, Value::Array(list.clone()));
            let items = list.clone();
            // 遍历数组中的每个元素
            for item in items {
                log::debug!("Processing item: {:?}", item);

                context.insert(element_key.clone(), item);

                if let Some(child) = children{
                    for child_token in child {
                        let result = parse_token(child_token,context);
                        if let Some(text) = result {

                            let value = variable_parse::normalize_variable_syntax(text.as_str(),context);

                            if let Some(value) = text_parse::parse_string(&value) {
                                output.push_str(&value);
                            }else{
                                output.push_str(&value);
                            }

                        }
                    }
                }

            }
        }
        return Some(output);
    }

    None
}

