use std::collections::HashMap;
use regex::Regex;
use serde_json::Value;
use crate::node;
use crate::node::node_parse::ExpressionNode;

pub fn new_node_trim(text: &str) -> ExpressionNode {
    ExpressionNode::TextNode {
        text: text.trim().to_string(),
    }
}


pub fn new_node(text: &str) -> ExpressionNode {
    if is_wrapped_with_newlines(text) && text.len()>1 {
        return ExpressionNode::TextNode {
            text:  remove_single_leading_newline(text).to_string(),
        };
    }
    ExpressionNode::TextNode {
        text: text.to_string(),
    }
}



fn is_wrapped_with_newlines(input: &str) -> bool {
    input.starts_with('\n') && input.ends_with('\n')
}


fn remove_single_leading_newline(input: &str) -> &str {
    if input.starts_with('\n') {
        &input[1..]
    } else {
        input
    }
}

pub fn node_to_string(node: &ExpressionNode,context:&HashMap<String, Value>) -> Option<String> {
    if let ExpressionNode::TextNode { text } = node {
        if text.is_empty() {
            return None;
        }
        // 直接返回文本节点的内容
        return Some(normalize_variable_syntax(text,context));
    }

    None
}

// 编译正则表达式
// lazy_static::lazy_static! {
//     static ref RE: Regex = Regex::new(r"\$\{ *([^}]+) *\}").unwrap();
// }
//
// pub fn normalize_variable_syntax( input: &str,context:&HashMap<String, Value>) -> String {
//     // 如果字符串中不包含 ${，直接返回输入字符串
//     if !input.contains("${") {
//         return input.to_string();
//     }
//
//     // 正则表达式匹配 ${} 内的内容，允许有空格
//     let re = Regex::new(r"\$\{ *([^}]+) *\}").unwrap();
//
//     // 使用正则表达式进行替换，去掉空格
//     RE.replace_all(input, |caps: &regex::Captures| {
//         let key = caps[1].trim(); // 提取变量名并去除空格
//         match context.get(key) {
//             Some(value) => match value {
//                 Value::String(s) => s.clone(), // 如果是 String 类型，直接返回内容，不加引号
//                 Value::Number(n) => n.to_string(), // 如果是数字，转换为字符串
//                 Value::Bool(b) => b.to_string(),   // 如果是布尔值，转换为字符串
//                 _ => format!("{}", value), // 其他类型，直接转换为字符串
//             },
//             None => format!("${{{}}}", key),  // 如果没找到，返回原始变量格式
//         }
//     })
//         .to_string()
// }

lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r"\$\{ *([^}]+) *\}|\$([a-zA-Z_][a-zA-Z0-9_]*)").unwrap(); // 支持 $age 和 ${age}
}

pub fn normalize_variable_syntax(input: &str, context: &HashMap<String, Value>) -> String {
    // 使用正则表达式进行替换
    RE.replace_all(input, |caps: &regex::Captures| {
        // 检查是 ${} 形式还是 $ 形式
        let key = if let Some(key) = caps.get(1) {
            key.as_str().trim()
        } else if let Some(key) = caps.get(2) {
            key.as_str()
        } else {
            return String::new(); // 如果没有匹配到有效的变量，返回空字符串
        };

        // 查找对应的值
        match context.get(key) {
            Some(value) => match value {
                Value::String(s) => s.clone(), // 如果是 String 类型，直接返回内容，不加引号
                Value::Number(n) => n.to_string(), // 如果是数字，转换为字符串
                Value::Bool(b) => b.to_string(),   // 如果是布尔值，转换为字符串
                _ => format!("{}", value), // 其他类型，直接转换为字符串
            },
            None => format!("${{{}}}", key), // 如果没找到，返回原始变量格式
        }
    })
        .to_string()
}