use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;
use serde_json::{Map, Number, Value};
use crate::expression::{expr_eval};
use crate::parse::{update_content, variable_parse};
use crate::token::token_parse::Tokenizer;
use evalexpr::{ Value as EvalValue,};



pub fn set_parse(token :&Tokenizer, context: &mut HashMap<String, Value>) {

    if let Tokenizer::Set { key,value } = token {
        // println!("key:{} vlaue:{}",key,value);
        let k = variable_parse::extract_variable(&key);
        let mut v = variable_parse::normalize_variable_syntax(value.as_str(),context);

        log::debug!("set key:{:?} value:{:?}",k,v);

        if let Some(key) = k{

            if expr_eval::is_valid_expression(v.as_str()) {
                if let Ok(value) = expr_eval::eval_value(v.as_str()){
                    match value {
                        EvalValue::String(_) => {
                            if let Ok(val) =value.as_string(){
                                update_content(context, &key, Value::String(val));
                            }
                        }
                        EvalValue::Float(_) => {
                            if let Ok(val) =value.as_float(){
                                update_content(context, &key, Value::from(val));
                            }
                        }
                        EvalValue::Int(_) => {
                            if let Ok(val) =value.as_int(){
                                update_content(context, &key, Value::from(val));
                            }
                        }
                        EvalValue::Boolean(_) => {
                            if let Ok(val) =value.as_boolean(){
                                update_content(context, &key, Value::Bool(val));
                            }
                        }
                        _ =>{

                        }
                    }
                    return;
                }
            }

            let new_value = if let Ok(parsed_bool) = v.parse::<bool>() {
                Value::Bool(parsed_bool)
            } else if let Ok(parsed_array) = serde_json::from_str::<Vec<Value>>(v.as_str()) {
                let len = parsed_array.len();
                update_content(context, format!("{}.size",&key).as_str(), Value::Number(Number::from(len)));
                Value::Array(parsed_array)
            } else if let Ok(map) = serde_json::from_str::<Map<String, Value>>(v.as_str()) {
                let len = map.len();
                update_content(context, format!("{}.size",&key).as_str(), Value::Number(Number::from(len)));
                Value::Object(map)
            } else if let Ok(string) = v.parse::<String>(){
                Value::String(string)
            } else if let Ok(parsed_int) = v.parse::<i64>() {
                Value::Number(Number::from(parsed_int))
            } else if let Ok(parsed_float) = v.parse::<f64>() {
                Value::Number(Number::from_f64(parsed_float).unwrap())
            } else {
                Value::String(v)
            };

            log::debug!("---------------------------{:#?}",context);
            update_content(context, &key, new_value);

        }

    }

}

