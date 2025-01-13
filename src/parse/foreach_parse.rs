use std::collections::HashMap;
use std::fmt::format;
use serde_json::{json, Map, Number, Value};
use serde_json::Value::Object;
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



        if let Some(value) = context.get(&collection_key){

            if let Value::Object(map) = value {
                let size =  map.len();
                let hash_map:serde_json::Map<String, Value>  = map.clone();

                let mut index = 0;
                let mut count    = 1;
                let mut has_next = true;
                let mut first = true;
                let mut  last = true;
                for mm in hash_map{
                    let key = mm.0;
                    let value = mm.1;

                    if index==0 {
                        first = true;
                        last = false;
                    }
                    if index+1 == size {
                        has_next = false;
                        last = true;
                    }

                    update_content(context, "foreach.count", Value::Number(Number::from(count)));
                    update_content(context, "foreach.first", Value::Bool(first));
                    update_content(context, "foreach.hasNext", Value::Bool(has_next));
                    update_content(context, "foreach.index", Value::Number(Number::from(index)));
                    update_content(context, "foreach.last", Value::Bool(last));

                    let mut mv: serde_json::Map<String, Value> =Map::<String,Value>::new();
                    mv.insert(key.to_string(),value.clone());

                    update_content(context, &element_key, Value::Object(mv));
                    update_content(context, format!("{}.key",&element_key).as_str(), Value::String(key.to_string()));
                    update_content(context, format!("{}.value",&element_key).as_str(), value.clone());

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
                    index += 1;
                }

            }else if let Value::Array(list) = value{
                let items = list.clone();
                let size = list.len();

                log::debug!("foreach context:{:?}",context);
                let mut index = 0;
                let mut count    = 1;
                let mut has_next = true;
                let mut first = true;
                let mut  last = true;
                for item in items {

                    log::debug!("foreach item:{:?}",&item);
                    log::debug!("foreach value is map:{:?}",item.is_object());

                    if index==0 {
                        first = true;
                        last = false;
                    }else{
                        first = false;
                    }
                    if index+1 == size {
                        has_next = false;
                        last = true;
                    }

                    if item.is_object() {
                        let map = item.as_object().unwrap();
                        for (key,value) in map {
                            update_content(context, format!("{}.{}", &element_key, &key).as_str(), value.clone());
                        }
                    }

                    update_content(context, &element_key, item);
                    update_content(context, "foreach.count", Value::Number(Number::from(count)));
                    update_content(context, "foreach.first", Value::Bool(first));
                    update_content(context, "foreach.hasNext", Value::Bool(has_next));
                    update_content(context, "foreach.index", Value::Number(Number::from(index)));
                    update_content(context, "foreach.last", Value::Bool(last));
                    update_content(context, format!("{}.index",&element_key).as_str(), Value::Number(Number::from(index)));

                    log::debug!("foreach context:{:#?}",&context);

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


                    log::debug!("output:{:#?}",&output);
                    index += 1;
                    count+=1;
                }

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
