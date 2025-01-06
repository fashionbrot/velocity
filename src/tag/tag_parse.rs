use regex::{Regex, escape};
use std::collections::{BTreeSet, HashMap, HashSet};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use crate::node::{foreach_node, if_node, text_node};
use crate::node::node_parse::ExpressionNode;

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
    pub child:Option<Vec<TagFinalPosition>>,
    pub else_list:Option<Vec<TagFinalPosition>>
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

pub fn build_tag_tree(mut tags: Vec<TagFinalPosition>) -> Option<Vec<TagFinalPosition>> {
    if tags.is_empty() {
        return None;
    }
    // 按 start 排序（降序），避免不必要的重复排序
    tags.sort_by_key(|tag| std::cmp::Reverse(tag.start));
    println!("tag---------\n{:#?}", tags);

    let mut result = Vec::new();
    // let mut processed = vec![false; tags.len()]; // 标记哪些标签已经被处理过
    let mut list = Vec::new();

    // 遍历所有标签
    for i in 0..tags.len() {

        let current_tag = &tags[i];
        let mut current_tag = current_tag.clone();

        // 获取当前标签的子标签
        let children = get_child(&list, &current_tag);
        if !children.is_empty() {
            children.iter().for_each(|child| {
                if let Some(index) = list.iter().position(|x| x.start == child.start && x.end == child.end) {
                    list.swap_remove(index);
                }
            })
        }
        current_tag.child = Some(children);

        // 将当前标签加入 list
        list.push(current_tag.clone());

        // 判断当前标签是否为根标签
        let is_parent = tags.iter().any(|m| m.is_parent(&current_tag));
        // println!("--is_parent:{:?} {:?}", is_parent, current_tag);

        if !is_parent {
            // 如果当前标签没有子标签，将它加入结果中
            println!("result push----------------{:#?}", current_tag);
            result.push(current_tag);
        }
    }

    println!("111111111111-------------\n{:#?}", result);
    if result.is_empty() {
        return None;
    }



    let mut data_list = result.clone();
    let mut remove_map: HashMap<usize,usize> = HashMap::new();
    //todo 需要放到上面去
    for  tag in &mut data_list {
        if tag.tag !="#if"  {
            continue;
        }
        let mut else_node_list = vec![];
        let mut current_end = tag.end;

        println!("current_end: {} ",current_end);
        while let Some((else_node,index)) = get_else_node(&result, current_end) {
            else_node_list.push(else_node.clone());
            current_end =else_node.end;
            remove_map.insert(else_node.start,else_node.end);
        }

        tag.else_list = Some(else_node_list);
    }

    if !remove_map.is_empty() {
        remove_map.iter().for_each(|(start, end)| {
            if let Some(index) = data_list.iter().position(|node| node.start==*start && node.end==*end){
                data_list.swap_remove(index);
            }
        })
    }

    data_list.sort_by_key(|tag| tag.start);
    Some(data_list)
}


// pub fn get_else_node(tags: &[TagFinalPosition], current_end: usize) -> Option<&TagFinalPosition> {
//     tags.iter()
//         .find(|list_tag| list_tag.start == current_end) // 直接返回引用
// }

pub fn get_else_node(tags: &[TagFinalPosition], current_end: usize) -> Option<(&TagFinalPosition, usize)> {
    // 使用 enumerate 获取索引和值
    tags.iter()
        .enumerate() // 获取每个元素的索引和值
        .find(|(_, list_tag)| list_tag.start == current_end) // 根据条件找到符合的元素
        .map(|(index, tag)| (tag, index)) // 返回元素引用和其索引
}


// pub fn get_else_node(tags: &[TagFinalPosition], current_tag: &TagFinalPosition) -> Option<TagFinalPosition> {
//     let else_nodes: Vec<TagFinalPosition>  = tags.iter().filter(|list_tag| list_tag.end == current_tag.start).cloned().collect();
//     if !else_nodes.is_empty() {
//         if let Some(else_node) = else_nodes.first() {
//             Some(else_node)
//         }
//     }
//     None
// }



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

    pub static ref TAGS: Vec<&'static str> = {
        let mut tags = Vec::new();
        tags.push("#if");
        tags.push("#elseif");
        tags.push("#else");
        tags.push("#foreach");
        tags.push("#end");
        tags
    };


        // 定义静态正则表达式模式，避免每次计算
    pub static ref TAGS_PATTERN: Regex = {
        // 生成正则表达式模式
        let pattern = TAGS.iter()
            .map(|tag| escape(*tag)) // 转义标签
            .collect::<Vec<String>>()
            .join("|"); // 使用 | 连接标签
        println!("pattern-------------{:?}" ,pattern);
        Regex::new(&format!(r"({})", pattern)).unwrap() // 返回正则表达式
    };
}

pub fn calculate_tag_positions(template: &str) -> Vec<TagPosition> {
    let mut tag_positions = Vec::new(); // 用来存储标签位置

    // 查找所有匹配的标签
    for (index, capture) in TAGS_PATTERN.find_iter(template).enumerate() {
        println!("calculate_tag_positions {} - {} index:{}", index, capture.as_str(),capture.start());
        if capture.as_str() =="#else" {
            tag_positions.push(TagPosition {
                tag: "#end".to_string(),
                index: capture.start(), // 获取标签的字符下标
            });
        }else if capture.as_str() =="#elseif"{
            tag_positions.push(TagPosition {
                tag: "#end".to_string(),
                index: capture.start(), // 获取标签的字符下标
            });
        }
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
        println!("1111{:?}", tag_position);
        if TAGS.contains(&tag_position.tag.as_str()) {
            if tag_position.tag.starts_with("#end")  {
                // 如果是 #end，尝试从栈中弹出一个开始标签
                if let Some(start_tag) = stack.pop() {
                    let tag_final_position = TagFinalPosition {
                        tag: start_tag.tag.clone(),
                        start: start_tag.index,
                        end: tag_position.index,
                        child: None,
                        else_list:None
                    };
                    final_positions.push(tag_final_position);
                } else {
                    println!("------------------------------{:?}", tag_position);
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


pub fn parse_template(start:usize, template:&str, tags: &Vec<TagFinalPosition>)-> Option<Vec<ExpressionNode>>{
    // println!("template:{}", template);
    // println!("tags: {:?}", tags);
    let mut node_list = vec![];
    if template.is_empty()  {
        return None;
    }

    let mut current_start = start;
    let first = start==0;
    let mut template_end = template.len();

    for i in 0..tags.len() {
        let tag = &tags[i];
        let tag_start = tag.start;
        let tag_end = tag.end;
        let tag_child = &tag.child;
        let tag_len = tag.tag.len();
        let else_list = &tag.else_list;
        println!(" tag: {} tag_len:{}",tag.tag,tag_len);
        // if current_start>0 {
        //     current_start = current_start+(tag_len+1);
        // }

        println!("start {} -end  {} ", current_start,tag_start);
        if current_start<tag_start {
            let text =    &template[current_start..tag_start];
            if let Some(text_node) =text_node::new_node(text) {
                node_list.push(text_node);
            }
            println!("first-tag_first {:?}", text);
        }



        println!("tag_start:{} tag_end:{}",tag_start,tag_end+3);
        let tag_text = &template[tag_start..=tag_end+3];
        println!("tag_text:{:?}", tag_text);


        let mut child_node_list:Option<Vec<ExpressionNode>> = None;
        if let Some(child) = tag_child {
            if let Some(pos) = tag_text.find(')') {
                let child_start = tag_start+pos+1;
                let child_end = tag_end ;
                // println!("tag_start:{} tag_end:{}",tag_start,tag_end);
                println!("child_start:{:?} child_end:{}", child_start,child_end);
                // println!("total:{}", template.len());
                let child_text = &template[child_start..child_end];
                println!("child_text:{:?}", child_text);

                if child.is_empty() {
                    if let Some(text_node) =text_node::new_node(child_text) {
                        child_node_list = Some(vec![text_node]);
                    }

                }else{
                    child_node_list = parse_template(tag_start+pos+1,template, child);
                }
            } else {
                if tag.tag == "#else" {
                    let child_text = &template[tag_start+tag_len..tag_end];
                    if let Some(text_node) =text_node::new_node(child_text) {
                        child_node_list = Some(vec![text_node]);
                    }
                }
                //todo 抛出错误
            }
        }

        let mut else_data_list = vec![];
        if let Some(else_list  ) = else_list {
            if !else_list.is_empty() {



                for else_tag in else_list {

                    let tag_start = else_tag.start;
                    let tag_end = else_tag.end;

                    if else_tag.tag == "#elseif" {
                        let else_text = &template[else_tag.start..else_tag.end];

                        println!("else_tag-----------------------{:?}  else_text:{:?}", else_tag,else_text);

                        let mut else_child_node_list:Option<Vec<ExpressionNode>> = None;
                        if let Some(pos) = else_text.find(')') {
                            let child_start = tag_start+pos+1;
                            let child_end = tag_end ;

                            let else_child_text = &template[child_start..child_end];
                            println!("if_else_child_start:{:?} if_else_child_end:{} if_else_child_text:{:?}" , child_start,child_end,else_child_text);
                            if let Some(child) = &else_tag.child{
                                if  child.is_empty() {
                                    if let Some(text_node) =text_node::new_node(else_child_text) {
                                        else_child_node_list = Some(vec![text_node]);
                                    }
                                }else{
                                    else_child_node_list = parse_template(tag_start+pos+1,template, child);
                                }
                            }
                        }

                        let condition = get_if_condition(else_text);
                        if let Some(condition) = condition {
                            else_data_list.push(ExpressionNode::IfNode {
                                condition: condition.parse().unwrap(),
                                children: else_child_node_list,
                                else_list: None
                            });
                        }else{
                            //todo 异常
                        }

                    }else if else_tag.tag == "#else" {

                        let mut else_child_node_list:Option<Vec<ExpressionNode>> = None;
                        let pos = else_tag.tag.len();
                        let child_start = tag_start+pos;
                        let child_end = tag_end ;
                        // println!("tag_start:{} tag_end:{}",tag_start,tag_end);

                        // println!("total:{}", template.len());
                        let child_text = &template[child_start..child_end];
                        println!("else_child_start:{:?} else_child_end:{} child_text:{:?}", child_start,child_end,child_text);
                        if let Some(child) = &else_tag.child{
                            if  child.is_empty() {
                                if let Some(text_node) =text_node::new_node(child_text) {
                                    else_child_node_list = Some(vec![text_node]);
                                }
                            }else{
                                else_child_node_list = parse_template(tag_start+pos,template, child);
                            }
                        }

                        else_data_list.push(ExpressionNode::IfNode {
                            condition: "true".to_string(),
                            children: else_child_node_list,
                            else_list: None
                        });

                    }
                }
            }
        }


        println!("-------------------------------------------{:?}", tag);
        if tag.tag == "#if" {


            let condition = get_if_condition(tag_text);
            println!("-----------------------------------------condition:{:?}",condition);
            if let Some(condition) = condition {
                node_list.push(ExpressionNode::IfNode {
                    condition: condition.parse().unwrap(),
                    children: child_node_list,
                    else_list: Some(else_data_list)
                });
            }else {
                //todo 提示解析异常
            }
            // if  tag.tag=="#else" && condition.is_none()  {
            //     node_list.push(ExpressionNode::IfNode {condition:"false".to_string(), children: child_node_list });
            // }else{
            //
            // }

        }else if tag.tag == "#foreach" {
            let condition = get_if_condition(tag_text);
            if let Some(condition) = condition {
                if let Some((left, right)) = get_foreach_condition(condition) {
                    node_list.push(ExpressionNode::ForeachNode {
                        collection: left,
                        element: right,
                        children: child_node_list,
                    });
                } else {
                    return None;
                }
            }else {
                //todo 提示解析异常
            }
        }


        current_start = tag_end+4;
        println!("{}", current_start);

        if first && i>0  &&i == tags.len()-1 {
            if current_start>0 {
                current_start = current_start+(tag_len+1);
            }
            println!("start {} -end  {} ", tag_end,template_end);
            let text = &template[tag_end+4..template_end];
            println!("tag_end - template_last:{:?}", text);

            if let Some(text_node) =text_node::new_node(text) {
                node_list.push(text_node);
            }
            // node_list.push(text_node::new_node(text));
        }


        //todo 打开
        // let tag_node = ExpressionNode::new_node(tag, child_node_list);
        // if let Some(node) = tag_node {
        //     node_list.push(node);
        // }
    }

    Some(node_list)
}


pub fn get_root_text(template:&str,start:usize,end:usize) -> &str{
    println!("Tag start: {}, end: {}", start, end);
    &template[start..end]
}


fn get_if_condition(input: &str) -> Option<&str> {
    // 查找 'if' 后面的 '(' 和第一个 ')'
    if let Some(start) = input.find('(') {
        if let Some(end) = input[start..].find(')') {
            // 返回括号内的内容
            return Some(&input[start + 1..start + end]);
        }
    }
    None
}

fn get_foreach_condition(input: &str) -> Option<(String, String)> {
    // 去掉字符串两端的空白字符
    let trimmed_input = input.trim();
    // 查找 'in' 的位置
    if let Some(in_index) = trimmed_input.find("in") {
        // 提取 'in' 之前和之后的内容
        let left = trimmed_input[..in_index].trim().to_string();
        let right = trimmed_input[in_index + 2..].trim().to_string();

        return Some((left, right));
    }

    None
}


#[cfg(test)]
mod tests {
    use crate::tag::tag_parse::{build_tag_tree, calculate_tag_final_positions, calculate_tag_positions, parse_template, TagFinalPosition};

    #[test]
    fn test1() {
        let template = r#"#if 你大爷 #end""#;

        let result =  calculate_tag_positions(template);
        // println!("{:#?}", result);
        let final_positions = calculate_tag_final_positions(result);

        let tree = build_tag_tree(final_positions.unwrap());
        println!("{:?}", tree);

        let result =Option::Some(vec![
            TagFinalPosition {
                tag: "#if".to_string(),
                start: 0,
                end: 14,
                child: Some(Vec::new()) ,// 指定空的 Vec
                else_list:None
            }
        ]);

        assert_eq!(result, tree);
    }


    #[test]
    fn parse_template_test(){
        let template =
r#"第一行
#if($lombokEnable)
import lombok.*;
#end
第三行
#if($lombokEnable)
    #foreach($field in $tableFieldList)

    private $field.attrType $field.variableAttrName;#end
#end

第八行
"#;
//         let template =
//             r#"第一行
// #if($lombokEnable)
// 第二行
// 第三行
// #end"#;
        let template =r#"
    #if($lombokEnable)import lombok.*;#end
#if($lombokEnable)
import lombok.*;
#end
#if($lombokEnable)
#foreach($field in $tableFieldList)
    第二行
#end
#end
"#;


        let result =  calculate_tag_positions(template);
        let final_positions = calculate_tag_final_positions(result);
        let tree = build_tag_tree(final_positions.unwrap());
        println!("{:#?}", tree);

        let node_list = parse_template(0,template, &tree.unwrap());
        println!("{:#?}", node_list);
    }

}