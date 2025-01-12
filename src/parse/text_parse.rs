use std::collections::HashMap;
use serde_json::Value;
use crate::parse::variable_parse;
use crate::token::token_parse::Tokenizer;

pub fn text_parse(token:&Tokenizer, context: &mut HashMap<String, Value>) -> Option<String> {

    if let Tokenizer::Text { text } = token {
        if text.is_empty() {
            return None;
        }
        let value = variable_parse::normalize_variable_syntax(text.as_str(),context);
        return parse_string(&value);
    }

    None
}


pub fn parse_string(text: &String) -> Option<String> {
    if text.len()>1 {
        if text.len()==2 && text.starts_with("\r\n") {
            return None;
        }

        if is_wrapped_with_crlf(text) {
            return Some(format!("{}\n",remove_surrounding_crlf(text)));
        }else{
            return Some(text.to_string());
        }
    }
    None
}

fn is_wrapped_with_crlf(input: &str) -> bool {
    let trimmed = input
        .trim_start_matches(' ')  // 去掉开头的空格
        .trim_end_matches(' ');   // 去掉结尾的空格
    if input.matches("\r\n").count()==1 {
        return false;
    }
    // println!("is_wrapped_with_crlf trimmed:{:?}",trimmed);
    trimmed.starts_with("\r\n") && trimmed.ends_with("\r\n")
}
fn remove_surrounding_crlf(input: &str) -> String {

    if input.starts_with("\r\n") && input.ends_with("\r\n") {
        return  input[2..input.len() - 2].to_string();
    }

    let start_ = input.find("\r\n");
    let end_ = input.rfind("\r\n");
    if start_.is_none() || end_.is_none() {
        return input.to_string();
    }
    let start = start_.unwrap();
    let end = end_.unwrap();

    if input.starts_with("\r\n") {
        let m_text = &input[2..end];
        let end_text = &input[end+2..];
        return format!("{}{}", m_text, end_text)
    }else if input.ends_with("\r\n") {
        let start_text = &input[0..start];
        let m_text = &input[(start+2)..end];
        return format!("{}{}", start_text, m_text)
    }

    let start_text = &input[0..start];
    let m_text = &input[(start+2)..end];
    let end_text = &input[end+2..];

    format!("{}{}{}", start_text, m_text, end_text)
}
