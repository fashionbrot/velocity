use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;

lazy_static! {
    // 同时匹配 ${} 包裹和 $ 开头的变量
    static ref VARIABLE_RE: Regex = Regex::new(r"\$\{([a-zA-Z_][a-zA-Z0-9_\.]*)\}|(\$[a-zA-Z_][a-zA-Z0-9_]*)").unwrap();
}

pub fn normalize_variable_syntax(input: &str, context: &mut HashMap<String, Value>) -> String {
    // 使用正则表达式进行替换
    VARIABLE_RE.replace_all(input, |caps: &regex::Captures| {
        // 提取变量名
        let key = if let Some(key) = caps.get(1) {
            key.as_str().to_string()
        } else if let Some(key) = caps.get(2) {
            key.as_str()[1..].to_string()
        } else {
            String::new()
        };
        // 查找对应的值
        match context.get(&key) {
            Some(value) => match value {
                Value::String(s) => s.clone(),
                Value::Number(n) => n.to_string(),
                Value::Bool(b) => b.to_string(),
                _ => format!("{}", value),
            },
            None => format!("${{{}}}", key),
        }
    })
        .to_string()
}


// lazy_static! {
//     static ref VARIABLE_RE: Regex = Regex::new(r"\$\{ *([a-zA-Z_][a-zA-Z0-9_\.]*) *\}|\$([a-zA-Z_][a-zA-Z0-9_]*)").unwrap(); // 支持 $foreach.index 和 ${foreach.index}
// }
//
// pub fn normalize_variable_syntax(input: &str, context: &mut HashMap<String, Value>) -> String {
//     // 使用正则表达式进行替换
//     VARIABLE_RE.replace_all(input, |caps: &regex::Captures| {
//         // 检查是 ${} 形式还是 $ 形式
//         let key = if let Some(key) = caps.get(1) {
//             key.as_str().trim()
//         } else if let Some(key) = caps.get(2) {
//             key.as_str().trim()
//         } else {
//             return String::new(); // 如果没有匹配到有效的变量，返回空字符串
//         };
//
//         // 查找对应的值
//         match context.get(key) {
//             Some(value) => match value {
//                 Value::String(s) => s.clone(), // 如果是 String 类型，直接返回内容，不加引号
//                 Value::Number(n) => n.to_string(), // 如果是数字，转换为字符串
//                 Value::Bool(b) => b.to_string(),   // 如果是布尔值，转换为字符串
//                 _ => format!("{}", value), // 其他类型，直接转换为字符串
//             },
//             None => format!("${{{}}}", key), // 如果没找到，返回原始变量格式
//         }
//     })
//         .to_string()
// }



pub  fn extract_variable(input: &String) -> Option<String> {
    // 使用懒加载正则，避免每次调用都编译正则
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(r"^\$\{?(.*?)\}?$").unwrap();
    }

    // 尝试匹配并提取变量名
    RE.captures(&input)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().trim().to_string()))
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use serde_json::Value;
    use crate::parse::variable_parse::normalize_variable_syntax;

    #[test]
    fn test_normalize_variable_syntax() {
        let mut context:HashMap<String,Value> = HashMap::new();
        context.insert("foreach.index".to_string(), Value::Number(1.into()));

        let input = "$foreach.index ";
        let result = normalize_variable_syntax(input, &mut context);
        println!("{}", result); // 输出：1
    }
}