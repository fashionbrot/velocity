use regex::Regex;
use std::collections::HashMap;
use std::ops::Add;
use crate::node::node_parse::ExpressionNode;

pub mod tag;
pub mod node;

// 编译正则表达式
lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r"\$\{ *([^}]+) *\}").unwrap();
}

#[derive(Debug)]
pub struct VelocityEngine {
    context: HashMap<String, String>,
    // 存储变量的上下文
    tag_handlers: Vec<&'static str>,
}


impl ExpressionNode {
    fn new(template: String, engine: VelocityEngine) -> Option<Vec<ExpressionNode>> {
        let mut node_list = vec![];
        let mut lines = template.lines().collect::<Vec<&str>>(); // 按行分割模板

        let mut read_line = 0;
        while read_line < lines.len() {
            let mut line = lines[read_line].to_string(); // 获取当前行

            // 匹配标签
            let (condition, tag) = engine.match_tag(&line);
            println!("condition: {} tag:{:?}", condition,tag);
            if condition {

                let tag_index = line.find(tag.unwrap()).unwrap();
                while read_line < lines.len() {
                    if let Some(end_index) = line.find("#end") {
                        println!("line:{} tag_index:{} end_index:{}",line,tag_index,end_index);
                        let all_text = lines[tag_index..=end_index].join("\n"); // 获取标签前的文本
                        println!("{}", all_text);
                        break;
                    }else {
                        line.push_str("\n");
                        line.push_str(&lines[read_line + 1]);
                        // 继续处理下一行
                        read_line += 1;
                    }
                }

                // if let Some(index) = line.find("#end") {
                //     // 如果找到 #END 标签
                //     let first_text = lines[0..tag_index].join("\n"); // 获取标签前的文本
                //     println!("{} {}", tag_index,first_text);
                //     let tag_text = lines[tag_index..index].join("\n"); // 获取标签文本
                //     let last_text = lines[index + 1..].join("\n"); // 获取标签后的文本
                //
                //     println!("--- {} {} {}", first_text, tag_text, last_text);
                //
                //     // 在此根据需求决定如何处理
                // } else {
                //     // 如果没有 #END 标签，合并下一行
                //     if read_line + 1 < lines.len() {
                //         line.push_str(&lines[read_line + 1]);
                //         // 继续处理下一行
                //         read_line += 1;
                //     }
                // }


            } else {
                // 如果没有标签，直接将文本加入节点列表
                node_list.push(ExpressionNode::TextNode { text: line, });
            }

            // 继续处理下一行
            read_line += 1;
        }

        println!("{:?}", node_list); // 调试输出节点列表
        Some(node_list)
    }

    fn parse(node_list: Vec<ExpressionNode>) -> Option<String> {
        None
    }
}

impl VelocityEngine {
    // 创建新的 VelocityEngine 实例
    pub fn new() -> Self {
        VelocityEngine {
            context: HashMap::new(),
            tag_handlers: vec!["#if", "#else", "#elseif", "#foreach"],
        }
    }

    // 设置变量，支持任意类型
    pub fn insert<T: ToString>(&mut self, key: &str, value: T) {
        self.context.insert(key.to_string(), value.to_string());
    }

    // 渲染模板
    pub fn render(&self, template: &str) -> String {
        let mut output = template.to_string();
        let mut loop_depth = 0;

        // 递归处理 #if, #elseif, #else, #foreach 标签
        output = self.parse_template(&output, &mut loop_depth);

        output
    }

    // 递归解析模板中的标签
    fn parse_template(&self, template: &str, loop_depth: &mut usize) -> String {
        let mut output = String::new();
        let mut i = 0;

        let mut lines = template.lines().collect::<Vec<&str>>(); // 按行分割模板
        for line in &lines {
            println!("{:?}", line)
        }
        let mut loop_depth = 0;
        for line in &lines {
            let (match_flag, tag) = self.match_tag(line);

            println!("match_flag: {} tag:{:?}", match_flag,tag);
            // 如果匹配到标签
            if match_flag {
                if let Some(tag_str) = tag {
                    // 使用正则表达式查找标签
                    let re = Regex::new(&tag_str).unwrap();
                    if let Some(matched) = re.find(line) {
                        println!("{:?}", matched.start());
                        println!("{:?}", &line[0..2]);
                        // 获取标签之前的内容
                        let first_str = &line[0..matched.start()];
                        println!("str:{:?}", first_str);
                        output.push_str(first_str); // 直接拼接字符串
                    }
                }
                output.push_str("\n");
            } else {
                // 如果没有匹配到标签，直接把整行添加到输出中
                output.push_str(line);
                output.push_str("\n");
            }
        }

        output
    }

    // 提取标签内容
    fn extract_tag(&self, template: &str, tag_pos: usize) -> String {
        let mut tag = String::new();
        let mut i = tag_pos;
        while i < template.len()
            && !template[i..].starts_with(" ")
            && template[i..].starts_with("#")
        {
            tag.push(template[i..].chars().next().unwrap());
            i += 1;
        }
        tag
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
#[cfg(test)]
mod tests {
    use crate::{VelocityEngine};
    use crate::node::node_parse::ExpressionNode;

    #[test]
    fn test1() {
        let mut engine = VelocityEngine::new();
        engine.insert("name", "Rust");
        let input = "Hello ${name}, how are ${ name} today? ${name } is great!";
        let result = engine.normalize_variable_syntax(input);
        println!("{}", result);
        assert!(result == "Hello Rust, how are Rust today? Rust is great!".to_string())
    }

    #[test]
    fn main() {
        let mut engine = VelocityEngine::new();
        engine.insert("name", "John");
        engine.insert("age", 18); // 插入数字
        engine.insert("is_active", true); // 插入布尔值

        let template = r#"张三#if($age > 18)18岁了#end
#if($age > 18)
    18岁了
#end
#foreach($item in $list)
    #if($item == "Item2")
        <p>Special Item: $item</p>
    #else
        <p>Regular Item: $item</p>
    #end
#end
"#;

        let rendered = engine.render(template);
        println!("{}", rendered);
    }

    #[test]
    fn test2() {
        let mut engine = VelocityEngine::new();
        engine.insert("name", "John");
        engine.insert("age", 18); // 插入数字
        engine.insert("is_active", true); // 插入布尔值

        let template = r#"
#if($mybatisPlusEnable)
import com.baomidou.mybatisplus.annotation.*;
#end
import com.fasterxml.jackson.annotation.JsonFormat;
import org.springframework.format.annotation.DateTimeFormat;
#if($serialVersionUIDEnable)
import java.io.Serializable;
#end
#if($lombokEnable)
import lombok.*;
#end"#;

        let node_list = ExpressionNode::new(template.parse().unwrap(), engine);
        if let Some(list) = node_list {
            println!("{:?}", list)
        }
    }
}
