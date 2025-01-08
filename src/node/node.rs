use std::fmt::Debug;
use regex::Match;
use serde::{Deserialize, Serialize};
use crate::tag::tag_parse::TAGS_PATTERN;

#[derive(Debug, Serialize, Deserialize,Clone,PartialEq)]
pub struct NodePosition {
    pub tag_name: String,
    pub tag_start:u32,
    pub tag_end :u32,
}

#[derive(Debug, Serialize, Deserialize,Clone,PartialEq)]
pub struct PositionTree {
    pub start_name:String,
    pub end_name:String,
    pub start:u32,
    pub end:u32,
    pub child:Option<Vec<PositionTree>>,
    pub branch:Option<Vec<PositionTree>>
}


impl PositionTree {
    pub fn new(begin:NodePosition, finish:NodePosition)-> PositionTree {
        PositionTree {
            start_name:begin.tag_name,
            end_name: finish.tag_name,
            start: begin.tag_start,
            end: finish.tag_start,
            child: None,
            branch: None,
        }
    }


    pub fn is_root(&self, tags: &[PositionTree]) -> bool {
        if self.start_name =="#else" || self.start_name=="#elseif" {
            return false;
        }
        let start = self.start;
        let end = self.end ;
        for x in tags {
            if x.start < start && start < x.end{
                return false
            }
            if x.start <end && end < x.end {
                return false
            }
        }
        true
    }


}

impl NodePosition {
    pub fn new(tag_name: &str, tag_start: usize, tag_end: usize) -> NodePosition {
        NodePosition{
            tag_name: tag_name.to_string(),
            tag_start:tag_start as u32,
            tag_end:tag_end as u32,
        }
    }

    pub fn build(capture: &Match) -> NodePosition {
        let tag_start = capture.start();
        let tag_end = capture.end();
        let tag_name = capture.as_str();
        NodePosition::new(tag_name, tag_start, tag_end)
    }

    pub fn print(&self) {
        println!("position tag_name: {:<10} tag_start:{:<10} tag_end:{:<10}", self.tag_name,self.tag_start,self.tag_end);
    }
}




pub fn build_tree_from_template(template:&str) ->Result<Vec<PositionTree>, String> {
    if template.is_empty() {
        return Err("template empty".to_string()); ;
    }

    //生成开始结束标签
    let mut pair_position_list:Vec<PositionTree> = Vec::new();
    let mut stack:Vec<NodePosition> = Vec::new(); // 用来存储开始标签的索引
    for (index,capture) in TAGS_PATTERN.find_iter(template).enumerate() {
        let node_position = NodePosition::build(&capture);
        node_position.print();
        let tag_name = &node_position.tag_name;

        if tag_name == "#end" {
            if let Some(position) = stack.pop() {
                pair_position_list.push(PositionTree::new(position, node_position.clone()));
            } else {
                // 如果没有找到匹配的开始标签，说明不匹配，报错
                return Err(format!("Unmatched #end at index {}", tag_name));
            }
        }else if tag_name == "#else" {
            if let Some(position) = stack.pop() {
                pair_position_list.push(PositionTree::new(position, node_position.clone()));
                stack.push(node_position);
            }
        }else if tag_name == "#elseif" {
            if let Some(position) = stack.pop() {
                pair_position_list.push(PositionTree::new(position, node_position.clone()));
                stack.push(node_position);
            }
        }else {
            stack.push(node_position);
        }
    }
    println!("pair position list: {:#?}",pair_position_list);

    if pair_position_list.is_empty(){
        return Ok(Vec::new());
    }

    //生成树形结构
    pair_position_list.sort_by_key(|tag| std::cmp::Reverse(tag.start));
    println!("111111111111111111  pair position list: {:#?}",pair_position_list);
    let mut position_tree:Vec<PositionTree> = Vec::new();
    let mut position_temp:Vec<PositionTree>  = Vec::new();
    for pair in &pair_position_list {
        let mut p = pair.clone();
        let start = p.start;
        let end = p.end;

        println!("start: {:?} finish: {:?}", start, end);

        //获取子节点&& 排序
        // let child = get_child(&pair_position_list,&p);
        let child:Vec<PositionTree> = position_temp
            .iter()
            .filter(|t|  start< t.start && end> t.end)
            .cloned()
            .collect();
        println!("child: {:?}",child);
        // //删除子节点在 position_temp 中的数据
        if !child.is_empty() {
            child.iter().for_each(|child| {
                if let Some(index) = position_temp.iter().position(|x| x.start == child.start && x.end == child.end) {
                    position_temp.swap_remove(index);
                }
            });
            p.child = Some(child.clone());
        }


        //解析else elseif
        let mut branch_list = vec![];
        let mut current_end = end;
        while let Some(branch) = get_branch(&position_temp, current_end) {
            current_end =branch.end;
            branch_list.push(branch.clone());
            if let Some(index) = position_temp.iter().position(|x| x.start == branch.start && x.end == branch.end) {
                position_temp.swap_remove(index);
            }
        }
        if !branch_list.is_empty() {
            p.branch = Some(branch_list);
        }


        position_temp.push(p.clone());

        if p.is_root(&pair_position_list){
            println!("root -------------------------------{:?}",p);
            position_tree.push(p.clone());
        }
    }

    println!("position tree: {:#?}",position_tree);


    Ok(position_tree)
}

pub fn get_branch(tags: &[PositionTree], current_end: u32) -> Option<&PositionTree> {
    tags.iter()
        .find(|tag| tag.start == current_end)
}
