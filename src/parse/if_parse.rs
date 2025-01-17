use std::collections::HashMap;
use serde_json::Value;
use crate::expression::expr_eval;
use crate::parse::{text_parse, variable_parse};
use crate::token::token_parse;
use crate::token::token_parse::{IfBranch, Tokenizer};


pub fn if_parse(token:&Tokenizer, context:&mut HashMap<std::string::String, Value>) -> Option<String> {
    if let Tokenizer::If { branches} = token {

        let mut output = String::new();

        for branch in branches {
            if let IfBranch::If{condition,children,..} = branch {
                let if_condition = variable_parse::normalize_variable_syntax(condition.as_str(), context);

                if expr_eval::eval(&if_condition) {
                    // println!("if expression:{:?}",expression);
                    if let Some(child) = children {

                        for child_token in child {
                            let result = token_parse::parse_token(child_token, context);

                            if let Some(text) = result {
                                if let Some(value) = text_parse::parse_string(&text) {
                                    if !value.trim().is_empty() {
                                        output.push_str(&value);
                                    }
                                } else {
                                    output.push_str(&text);
                                }
                            }
                        }

                    }

                    if let Some(value) = text_parse::parse_string(&output) {
                        if !value.trim().is_empty() {
                            return Some(value);
                        }
                    } else {
                        return Some(output);
                    }

                }

            }
        }

        if let Some(value) = text_parse::parse_string(&output) {
            if !value.trim().is_empty() {
                return Some(value);
            }
        } else {
            return Some(output);
        }

    }
    None
}
