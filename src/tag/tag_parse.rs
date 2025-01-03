use std::alloc::System;
use regex::{Regex, escape};
use std::collections::HashSet;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TagPosition {
    pub tag: String,
    pub index: usize, // 字符下标
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagFinalPosition {
    pub tag: String,
    pub start: usize, // 开始下标
    pub end: usize,   // 结束下标
    // pub child:Option<Vec<TagPosition>>
}

// 使用 lazy_static 宏定义静态变量 tags
lazy_static! {
    pub static ref TAGS: HashSet<&'static str> = {
        let mut tags = HashSet::new();
        tags.insert("#if");
        tags.insert("#foreach");
        tags.insert("#end");
        tags
    };
}

pub fn calculate_tag_positions(template: &str) -> Vec<TagPosition> {
    let mut tag_positions = Vec::new(); // 用来存储标签位置

    // 生成正则表达式模式
    let pattern = TAGS.iter()
        .map(|tag| escape(*tag)) // 转义标签
        .collect::<Vec<String>>()
        .join("|"); // 使用 | 连接标签

    // 使用生成的正则表达式来匹配标签
    let re = Regex::new(&format!(r"({})", pattern)).unwrap();

    // 查找所有匹配的标签
    for (index, capture) in re.find_iter(template).enumerate() {
        println!("{} - {} index:{}", index, capture.as_str(),capture.start());
        tag_positions.push(TagPosition {
            tag: capture.as_str().to_string(),
            index: capture.start(), // 获取标签的字符下标
        });
    }

    tag_positions
}

pub fn calculate_tag_final_positions(tag_positions: Vec<TagPosition>) -> Result<Vec<TagFinalPosition>, String> {
    let mut final_positions = Vec::new();
    let mut stack:Vec<TagPosition> = Vec::new(); // 用来存储开始标签的索引

    for tag_position in tag_positions {
        if TAGS.contains(&tag_position.tag.as_str()) {
            if tag_position.tag.starts_with("#end") {
                // 如果是 #end，尝试从栈中弹出一个开始标签
                if let Some(start_tag) = stack.pop() {
                    let tag_final_position = TagFinalPosition {
                        tag: start_tag.tag.clone(),
                        start: start_tag.index,
                        end: tag_position.index,
                    };
                    final_positions.push(tag_final_position);
                } else {
                    // 如果没有找到匹配的开始标签，说明不匹配，报错
                    return Err(format!("Unmatched #end at index {}", tag_position.index));
                }
            } else {
                // 如果是开始标签，推入栈中
                stack.push(tag_position);
            }
        }
    }

    // 检查栈是否为空，如果不为空，说明有多余的未匹配的开始标签
    if !stack.is_empty() {
        return Err("There are unmatched #if or #foreach tags".to_string());
    }

    Ok(final_positions)
}

pub fn test(template:&str,tag_final_position: Result<Vec<TagFinalPosition>, String>) -> Result<Vec<TagPosition>, String> {
    if let Err(error) = tag_final_position {
        return Err(error);
    }

    let mut tag_final_position = tag_final_position?;
    tag_final_position.sort_by(|a, b| a.start.cmp(&b.start));
    println!("{:?}", tag_final_position);
    let mut start_index = 0;
    let mut tag_index = 0;
    let chars: Vec<char> = template.chars().collect();
    println!("{}", chars.len());
    while start_index < chars.len() {
        let tag_final_position = &tag_final_position[tag_index];
        let first_text:String = chars[start_index..tag_final_position.start].iter().collect();
        let tag_text:String = chars[tag_final_position.start + 1..tag_final_position.end].iter().collect();
        println!("first_text:{}",first_text);
        println!("tag_text:{}",tag_text);
        tag_index= tag_index + 1;
        start_index= tag_final_position.end+1;
        println!("start_index:{}",start_index);
    }


    Ok(Vec::new())
}

// pub fn get_child(template:&str,tag_final_position:Vec<TagFinalPosition>) -> Result<String, String> {
//
// }