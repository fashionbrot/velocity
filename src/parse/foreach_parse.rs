use std::collections::HashMap;
use serde_json::{Number, Value};
use crate::parse::{text_parse, variable_parse};
use crate::token::token_parse::{parse_token, Tokenizer};

pub fn foreach_parse(token:&Tokenizer, context:&mut HashMap<String, Value>) -> Option<std::string::String> {

    if let Tokenizer::Foreach { element,collection,children } = token {
        let mut output = String::new();

        let element_key = if let Some(key) = variable_parse::extract_variable(&element) {
            key.trim().to_string()
        } else {
            element.to_string()
        };


        let collection_key = if let Some(key) = variable_parse::extract_variable(&collection) {
            key.trim().to_string()
        } else {
            collection.to_string()
        };

        log::debug!("element_key:{} collection_key:{}",element_key,collection_key);
        // log::debug!("context:{:#?}", context);
        log::debug!("----------children:{:#?}", children);
        log::debug!("parse_foreach key:{:?} element:{} collection:{}",element_key,element,collection);
        // log::debug!("foreach collection_key:{:?}  collection_value:{:?}",&collection_key,&context.get_mut(&collection_key));



        // if let Some(value) = &context.get(&collection_key) {
        //     match value {
        //         Value::Array(list) => {
        //             // 处理数组
        //             let items = list.clone();
        //             let mut index = 0;
        //             for item in items {
        //
        //                 update_content(context, &element_key, item);
        //                 update_content(context, "foreach.index", Value::Number(Number::from(index)));
        //
        //                 let mut child_output = String::new();
        //                 if let Some(child) = children {
        //                     for child_token in child {
        //                         log::debug!("foreach children token:{:?}", &child_token);
        //                         if let Some(text) = parse_token(&child_token, context) {
        //                             let value = variable_parse::normalize_variable_syntax(text.as_str(), context);
        //                             child_output.push_str(&value);
        //                         }
        //                     }
        //                 }
        //
        //                 if let Some(text) = text_parse::parse_string(&child_output) {
        //                     output.push_str(&text);
        //                 } else {
        //                     output.push_str(&child_output);
        //                 }
        //
        //                 index += 1;
        //             }
        //         }
        //         Value::Object(map) => {
        //             // 处理对象
        //             for (key, value) in map {
        //
        //                 log::debug!("foreach child key:{:?} value:{:?}",&key,&value);
        //                 // update_content(context, &element_key, value);
        //                 // update_content(context, format!("{}",&element_key), value);
        //                 // update_content(context, "foreach.value", value.clone());
        //
        //                 let mut child_output = String::new();
        //                 if let Some(child) = children {
        //                     for child_token in child {
        //                         // log::debug!("foreach children token:{:?}", &child_token);
        //                         if let Some(text) = parse_token(&child_token.clone(), context) {
        //                             let value = variable_parse::normalize_variable_syntax(text.as_str(), context);
        //                             child_output.push_str(&value);
        //                         }
        //                     }
        //                 }
        //
        //                 if let Some(text) = text_parse::parse_string(&child_output) {
        //                     output.push_str(&text);
        //                 } else {
        //                     output.push_str(&child_output);
        //                 }
        //             }
        //         }
        //         _ => {
        //             log::warn!("Unsupported type for collection_key: {:?}", value);
        //         }
        //     }
        // }




        // // 从 context 中获取集合对象
        if let Some(Value::Array(list)) = context.get(&collection_key) {
            // 将集合对象更新到 context 中
            // context.insert(key, Value::Array(list.clone()));
            let items = list.clone();

            let mut index = 0;
            // 遍历数组中的每个元素
            for item in items {

                update_content(context, &element_key, item);
                update_content(context, "foreach.index", Value::Number(Number::from(index)));

                let mut child_output = String::new();
                if let Some(child) = children{
                    for child_token in child {
                        log::debug!("foreach children token:{:?}",&child_token);
                        let result = parse_token(child_token,context);
                        if let Some(text) = result {

                            let value = variable_parse::normalize_variable_syntax(text.as_str(),context);
                            child_output.push_str(&value);
                        }
                    }
                }

                if let Some(text) = text_parse::parse_string(&child_output){
                    output.push_str(&text);
                }else{
                    output.push_str(&child_output);
                }
                // log::debug!("Processing item: {:?}", item);
                log::debug!("context:{:#?}", context);
                index += 1;
            }


        }

        return Some(output);
    }

    None
}


// fn process_array(
//     list: &[Value],
//     element_key: String,
//     children: &Option<Vec<Tokenizer>>,
//     context: &mut HashMap<String, Value>,
//     output: &mut String,
// ) {
//     for (index, item) in list.iter().enumerate() {
//         update_content(context, element_key.to_string(), item.clone());
//         update_content(context, "foreach.index".to_string(), Value::Number(Number::from(index as i64)));
//
//         if let Some(child_tokens) = children {
//             render_children(child_tokens, context, output);
//         }
//     }
// }

// fn process_object(
//     map: &serde_json::Map<String, Value>,
//     element_key: String,
//     children: &Option<Vec<Tokenizer>>,
//     context: &mut HashMap<String, Value>,
//     output: &mut String,
// ) {
//     for (key, value) in map {
//         update_content(context, element_key.to_string(), Value::String(key.clone()));
//         update_content(context, "foreach.value".to_string(), value.clone());
//
//         if let Some(child_tokens) = children {
//             render_children(child_tokens, context, output);
//         }
//     }
// }
//
//

// fn render_children(
//     children: &[Tokenizer],
//     context: &mut HashMap<String, Value>,
//     output: &mut String,
// ) {
//     let mut child_output = String::new();
//
//     for child_token in children {
//         log::debug!("Processing child token: {:?}", child_token);
//         if let Some(text) = parse_token(child_token, context) {
//             let normalized_text = variable_parse::normalize_variable_syntax(&text, context);
//             child_output.push_str(&normalized_text);
//         }
//     }
//
//     if let Some(parsed_text) = text_parse::parse_string(&child_output) {
//         output.push_str(&parsed_text);
//     } else {
//         output.push_str(&child_output);
//     }
// }


fn update_content(content: &mut HashMap<String, Value>, key:&str, new_value: Value) {
    if let Some(value) = content.get_mut(key) {
        *value = new_value;
    } else {
        content.insert(key.to_string(), new_value);
    }
}
