use std::alloc::System;
use regex::{Regex, escape};
use std::collections::HashSet;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize,Clone,PartialEq)]
pub struct TagPosition {
    pub tag: String,
    pub index: usize, // 字符下标
}

#[derive(Debug, Serialize, Deserialize,Clone,PartialEq)]
pub struct TagFinalPosition {
    pub tag: String,
    pub start: usize,
    pub end: usize,
    pub child:Option<Vec<TagFinalPosition>>
}


impl TagFinalPosition {
    // 递归方法来构建树
    fn add_child(&mut self, child: TagFinalPosition) {
        if let Some(ref mut children) = self.child {
            children.push(child);
        } else {
            self.child = Some(vec![child]);
        }
    }

    fn is_parent(&self, other: &TagFinalPosition) -> bool {
        self.start < other.start && self.end > other.end
    }
    fn is_child(&self, other: &TagFinalPosition) -> bool {
        self.start > other.start && self.end < other.end
    }
}

pub fn build_tag_tree(mut tags: Vec<TagFinalPosition>) -> Vec<TagFinalPosition> {
    // 按 start 排序（降序），避免不必要的重复排序
    tags.sort_by_key(|tag| std::cmp::Reverse(tag.start));
    // println!("---------{:?}", tags);

    let mut result = Vec::new();
    // let mut processed = vec![false; tags.len()]; // 标记哪些标签已经被处理过
    let mut list = Vec::new();

    // 遍历所有标签
    for i in 0..tags.len() {
        // if processed[i] {
        //     continue; // 如果当前标签已经被处理过，跳过
        // }

        let current_tag = &tags[i];
        let mut current_tag = current_tag.clone();

        // 获取当前标签的子标签
        let children = get_child(&list, &current_tag);
        current_tag.child = Some(children);

        // 将当前标签加入 list
        list.push(current_tag.clone());

        // 判断当前标签是否为根标签
        let is_parent = tags.iter().any(|m| m.is_parent(&current_tag));
        // println!("--is_parent:{:?} {:?}", is_parent, current_tag);

        if !is_parent {
            // 如果当前标签没有子标签，将它加入结果中
            result.push(current_tag);
        }

        // processed[i] = true; // 标记该标签已经处理
    }

    // 按 start 排序升序
    result.sort_by_key(|tag| tag.start);
    result
}

// pub fn build_tag_tree(mut tags: Vec<TagFinalPosition>) -> Vec<TagFinalPosition> {
//     // 按 start 排序
//     // tags.sort_by_key(|tag| tag.start);
//     tags.sort_by_key(|tag| std::cmp::Reverse(tag.start));
//     println!("---------{:?}", tags);
//
//     let mut result: Vec<TagFinalPosition> = Vec::new();
//     let mut processed = vec![false; tags.len()]; // 标记哪些标签已经被处理过
//
//     let mut list = Vec::new();
//
//     for i in 0..tags.len() {
//         if processed[i] {
//             continue; // 如果当前标签已经被处理过，跳过
//         }
//
//         let mut current_tag = tags[i].clone();
//         let p = get_child(&list,&current_tag);
//         println!("22222-{:?}",p);
//         current_tag.child = Some(p);
//
//         list.push(current_tag.clone());
//
//
//         let count = tags.iter().filter(|m| m.is_parent(&current_tag)).count();
//         println!("--count:{:?} {:?}",count,current_tag);
//
//         if count == 0 {
//             // 将当前标签加入结果中
//             result.push(current_tag);
//         }
//
//         processed[i] = true; // 标记该标签已经处理
//     }
//
//     result.sort_by_key(|tag| tag.start);
//     result
// }


pub fn get_child(tags: &[TagFinalPosition], current_tag: &TagFinalPosition) -> Vec<TagFinalPosition> {
    // 遍历 tags 查找与当前标签匹配的子标签
    tags.iter()
        .filter(|&tag| tag.is_child(current_tag))
        .cloned()
        .collect()
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

        // 定义静态正则表达式模式，避免每次计算
    pub static ref TAGS_PATTERN: Regex = {
        // 生成正则表达式模式
        let pattern = TAGS.iter()
            .map(|tag| escape(*tag)) // 转义标签
            .collect::<Vec<String>>()
            .join("|"); // 使用 | 连接标签

        Regex::new(&format!(r"({})", pattern)).unwrap() // 返回正则表达式
    };
}

pub fn calculate_tag_positions(template: &str) -> Vec<TagPosition> {
    let mut tag_positions = Vec::new(); // 用来存储标签位置

    // 查找所有匹配的标签
    for (index, capture) in TAGS_PATTERN.find_iter(template).enumerate() {
        // println!("{} - {} index:{}", index, capture.as_str(),capture.start());
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
                        child: None
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
    println!("{:#?}", tag_final_position);
    let mut start_index = 0;
    let mut tag_index = 0;
    let chars: Vec<char> = template.chars().collect();
    println!("{}", chars.len());

    let mut temp = vec![];

    while start_index < chars.len() {
        let tag_final_position = &tag_final_position[tag_index];
        if temp.contains(&tag_final_position.start) {

        }
        temp.push(tag_final_position.start);
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

pub fn test2(template:&str,tag_final_position: Result<Vec<TagFinalPosition>, String>) -> Result<Vec<TagPosition>, String> {
    if let Err(error) = tag_final_position {
        return Err(error);
    }
    let mut tag_final_position = tag_final_position?;
    tag_final_position.sort_by(|a, b| a.start.cmp(&b.start));
    println!("{:#?}", tag_final_position);
    let mut start_index = 0;
    let mut tag_index = 0;
    let chars: Vec<char> = template.chars().collect();
    println!("{}", chars.len());

    let mut temp = vec![];

    for i in 0..tag_final_position.len() {
        let tag_position = &tag_final_position[i];
        let start = tag_position.start;
        let end = tag_position.end;
        println!("start:{} end:{}",start,end);
        if temp.contains(&start) {
            continue;
        }

        // let first_text:String = chars[start_index..tag_position.start].iter().collect();
        let tag_text:String = chars[tag_position.start ..tag_position.end+4].iter().collect();
        println!("");
        // println!("first_text:{}",first_text);
        println!("tag_text:{}",tag_text);
        println!("");
        temp.push(start);
    }


    Ok(Vec::new())
}

// pub fn get_child(template:&str,tag_final_position:Vec<TagFinalPosition>) -> Result<String, String> {
//
// }

#[cfg(test)]
mod tests {
    use crate::tag::tag_parse::{build_tag_tree, calculate_tag_final_positions, calculate_tag_positions, TagFinalPosition};

    #[test]
    fn test1() {
        let template = r#"#if 你大爷 #end""#;

        let result =  calculate_tag_positions(template);
        // println!("{:#?}", result);
        let final_positions = calculate_tag_final_positions(result);

        let tree = build_tag_tree(final_positions.unwrap());
        println!("{:?}", tree);

        let result = vec![
            TagFinalPosition {
                tag: "#if".to_string(),
                start: 0,
                end: 14,
                child: Some(Vec::new()) // 指定空的 Vec
            }
        ];

        assert_eq!(result, tree);
    }
}