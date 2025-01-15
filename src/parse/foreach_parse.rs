use std::collections::HashMap;
use std::fmt::format;
use serde::Deserializer;
use serde_json::{Map, Number, Value};
use crate::parse::{text_parse, update_content, variable_parse};
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

        // log::debug!("element_key:{} collection_key:{}",element_key,collection_key);
        // // log::debug!("context:{:#?}", context);
        // log::debug!("----------children:{:#?}", children);
        // log::debug!("parse_foreach key:{:?} element:{} collection:{}",element_key,element,collection);



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

                    update_content(context, format!("{}.count",&element_key).as_str(), Value::Number(Number::from(count)));
                    update_content(context, format!("{}.first",&element_key).as_str(), Value::Bool(first));
                    update_content(context, format!("{}.hasNext",&element_key).as_str(), Value::Bool(has_next));
                    update_content(context, format!("{}.index",&element_key).as_str(), Value::Number(Number::from(index)));
                    update_content(context, format!("{}.last",&element_key).as_str(), Value::Bool(last));

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
                    count += 1;
                }

            }else if let Value::Array(list) = value{
                let items:Vec<Value> = list.clone();
                let size = list.len();

                // log::debug!("foreach context:{:?}",context);
                let mut index = 0;
                let mut count    = 1;
                let mut has_next = true;
                let mut first = true;
                let mut  last = true;
                for item in items {

                    // log::debug!("foreach item:{:?}",&item);
                    // log::debug!("foreach value is map:{:?}",item.is_object());


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
                    }else if item.is_string() {
                        update_content(context, &element_key,Value::String(item.to_string()));
                    }else{
                        update_content(context, &element_key, item);
                    }


                    update_content(context, format!("{}.count",&element_key).as_str(), Value::Number(Number::from(count)));
                    update_content(context, format!("{}.first",&element_key).as_str(), Value::Bool(first));
                    update_content(context, format!("{}.hasNext",&element_key).as_str(), Value::Bool(has_next));
                    update_content(context, format!("{}.index",&element_key).as_str(), Value::Number(Number::from(index)));
                    update_content(context, format!("{}.last",&element_key).as_str(), Value::Bool(last));

                    // log::debug!("foreach context:{:#?}",&context);

                    let mut child_output = String::new();
                    if let Some(child) = children{
                        for child_token in child {
                            // log::debug!("foreach children token:{:?}",&child_token);
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


                    // log::debug!("output:{:#?}",&output);
                    index += 1;
                    count+=1;
                }

            }
        }

        return Some(output);
    }

    None
}


