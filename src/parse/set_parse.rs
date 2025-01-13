use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;
use serde_json::{Map, Number, Value};
use crate::expression::expression_evaluator;
use crate::parse::variable_parse;
use crate::token::token_parse::Tokenizer;

lazy_static!(

    static ref  MEVAL_REGEX:Regex = Regex::new(r"(\+|\-|\*|\/|\%|\^|&&|\|\||==|!=|sqrt|abs|exp|ln|sin|cos|tan|asin|acos|atan|atan2|sinh|cosh|tanh|asinh|acosh|atanh|floor|ceil|round|signum|max|min|pi|e)").unwrap();

);

fn contains_meval(input: &str) -> bool {
    // 正则表达式匹配数学运算符、逻辑运算符、函数和常量
    // 检查输入字符串是否包含匹配的内容
    MEVAL_REGEX.is_match(input)
}

pub fn set_parse(token :&Tokenizer, context: &mut HashMap<String, Value>) {

    if let Tokenizer::Set { key,value } = token {
        // println!("key:{} vlaue:{}",key,value);
        let k = variable_parse::extract_variable(&key);
        let mut v = variable_parse::normalize_variable_syntax(value.as_str(),context);

        log::debug!("set key:{:?} value:{:?}",k,v);

        if let Some(key) = k{

            if contains_comparators(v.as_str()) {
                if let Ok((new_value)) = expression_evaluator::evaluate_expression(v.as_str()){
                    update_content(context, &key, Value::Bool(true));
                    return;
                }
            }

            if contains_meval(v.as_str()) {
                if let Ok(new_value) = meval::eval_str(v.as_str()) {
                    update_content(context, &key, Value::Number(Number::from_f64(new_value).unwrap()));
                    return;
                }

                log::debug!("str:{:?} eval:{}",v,meval::eval_str(v.as_str()).err().unwrap());
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

fn update_content(content: &mut HashMap<String, Value>, key: &str, new_value: Value) {
    if let Some(value) = content.get_mut(key) {
        *value = new_value;
    } else {
        content.insert(key.to_string(), new_value);
    }
}

fn contains_comparators(input: &str) -> bool {
    let bytes = input.as_bytes();
    let len = bytes.len();

    for i in 0..len {
        match bytes[i] {
            b'<' | b'>' | b'=' | b'!' => {
                // 检查是否是双字符操作符
                if i + 1 < len && bytes[i + 1] == b'=' {
                    return true; // 匹配 <=, >=, ==, !=
                }
                // 单字符操作符 < 或 >
                if bytes[i] != b'=' && bytes[i] != b'!' {
                    return true;
                }
            }
            _ => {}
        }
    }

    false
}