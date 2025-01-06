use regex::Regex;
use std::collections::HashMap;
use std::ops::Add;
use serde_json::Value;
use crate::node::node_parse::ExpressionNode;
use crate::tag::tag_parse;

pub mod tag;
pub mod node;

pub mod expression;

// 编译正则表达式
lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r"\$\{ *([^}]+) *\}").unwrap();
}

#[derive(Debug)]
pub struct VelocityEngine {
    context: HashMap<String, Value>,
    // 存储变量的上下文
    tag_handlers: Vec<&'static str>,
}

impl VelocityEngine {
    // 创建新的 VelocityEngine 实例
    pub fn new() -> Self {
        VelocityEngine {
            context: HashMap::new(),
            tag_handlers: vec!["#if", "#foreach"],
        }
    }

    // 设置变量，支持任意类型
    pub fn insert(&mut self, key: &str, value: Value) {
        self.context.insert(key.to_string(), value);
    }

    // 渲染模板
    pub fn render(&self, template: &str) -> String {
        self.parse_template(template)
    }

    // 递归解析模板中的标签
    fn parse_template(&self, template: &str) -> String {
        let mut output = String::new();

        let result =  tag_parse::calculate_tag_positions(template);
        println!("tag_position_list:{:#?}",result);
        let final_positions = tag_parse::calculate_tag_final_positions(result);
        println!("final_positions:{:#?}",final_positions);
        let tree = tag_parse::build_tag_tree(final_positions.unwrap());
        // println!("tree:{:#?}",tree);

        let mut node_list = None;
        if let Some(tree) = tree {
            node_list = tag_parse::parse_template(0,template, &tree);
        }
        println!("node_list:{:#?}", node_list);
        if let Some(node_list) = node_list {
            for node in node_list {
                let content = &self.context;

                let result = ExpressionNode::get_node_text(node, content);
                if let Some(node_text) = result {
                    output.push_str(&node_text);
                }
            }
        }


        output
    }



    fn match_tag(&self, line: &str) -> (bool, Option<&str>) {
        for tag in self.tag_handlers.iter() {
            if line.contains(&*tag) {
                return (true, Some(&*tag)); // 如果命中标签，返回 true 和标签名
            }
        }
        (false, None) // 如果没有命中标签，返回 false 和 None
    }

    fn normalize_variable_syntax(&self, input: &str) -> String {
        // 如果字符串中不包含 ${，直接返回输入字符串
        if !input.contains("${") {
            return input.to_string();
        }

        // 正则表达式匹配 ${} 内的内容，允许有空格
        let re = Regex::new(r"\$\{ *([^}]+) *\}").unwrap();

        // 使用正则表达式进行替换，去掉空格
        RE.replace_all(input, |caps: &regex::Captures| {
            let key = caps[1].trim(); // 提取变量名并去除空格
            match self.context.get(key) {
                Some(value) => value.to_string(), // 如果找到了，返回对应的值
                None => format!("${{{}}}", key),  // 如果没找到，返回原始变量格式
            }
        })
        .to_string()
    }
}


pub fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(file_path)
}

#[cfg(test)]
mod tests {
    use crate::{VelocityEngine};

    #[test]
    fn test1() {
        let mut engine = VelocityEngine::new();
        // engine.insert("name", "Rust");
        let input = "Hello ${name}, how are ${ name} today? ${name } is great!";
        let result = engine.normalize_variable_syntax(input);
        println!("{}", result);
        assert!(result == "Hello Rust, how are Rust today? Rust is great!".to_string())
    }

    // #[test]
    // fn main() {
    //     let mut engine = VelocityEngine::new();
    //     engine.insert("name", "John");
    //     engine.insert("age", 18); // 插入数字
    //     engine.insert("is_active", true); // 插入布尔值
    //
    //     let template = r#"
    //     第二行
    //     #if($age > 18)
    //         18岁了，第三行
    //     #end
    //     第四行
    //     #foreach($item in $list)
    //         #if($item == "Item2")
    //             <p>Special Item: $item</p>
    //         #else
    //             <p>Regular Item: $item</p>
    //         #end
    //     #end
    //     "#;
    //
    //     let output = engine.render(template);
    //     println!("output:{:?}", output);
    // }


}
