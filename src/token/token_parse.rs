use meval::tokenizer::Token;
use serde::{Deserialize, Serialize};
use crate::node::{ PositionTree, TAGS_PATTERN};


#[derive(Debug)]
pub enum Tokenizer{
    Text {
        text: String,
    },
    Set{
        key: String,
        value: String,
    },
    If{
        branches: Vec<IfBranch>, // 条件分支，包括 If, ElseIf, Else
    },
    Foreach{
        element: String,
        collection: String,
        children: Vec<Tokenizer>
    }
}

#[derive(Debug)]
pub enum IfBranch {
    If {
        condition: String,
        children: Vec<Tokenizer>
    },
    ElseIf {
        condition: String,
        children: Vec<Tokenizer>
    },
    Else {
        children: Vec<Tokenizer>
    },
}

#[derive(Debug, Serialize, Deserialize,Clone,PartialEq)]
pub struct TokenPosition {
    pub first_name: String,
    pub first_start:usize,
    pub first_end :usize,
    pub last_name:String,
    pub last_start:usize,
    pub last_end:usize,
}
impl TokenPosition {

    pub fn new(first_name: &str, first_start: usize, first_end: usize, last_name: &str, last_start: usize, last_end: usize) -> Self {
        TokenPosition{
            first_name:first_name.to_string(),
            first_start,
            first_end,
            last_name:last_name.to_string(),
            last_start,
            last_end,
        }
    }
    pub fn build(start:&NodePosition,end:&NodePosition) -> Self {
        TokenPosition{
            first_name:start.name.to_string(),
            first_start:start.start,
            first_end:start.end,
            last_name:end.name.to_string(),
            last_start:end.start,
            last_end:end.end,
        }
    }
}

#[derive(Debug, Serialize, Deserialize,Clone,PartialEq)]
pub struct NodePosition {
    pub name: String,
    pub start:usize,
    pub end :usize,
}

impl NodePosition {
    pub fn new(name: &str,start:usize,end:usize) -> Self {
        NodePosition{
            name:name.to_string(),
            start,
            end
        }
    }
}




pub fn parse_token(template:&str)->Result<Vec<Tokenizer>,String>{
    log::debug!("parse_token start");
    if template.is_empty() {
        return Err("template empty".to_string()); ;
    }
    let mut tokens:Vec<Tokenizer> = Vec::new();

    // 生成开始结束标签
    let mut stack: Vec<NodePosition> = Vec::new(); // 用来存储开始标签的索引
    let mut captures: Vec<(usize, &str)> = TAGS_PATTERN.find_iter(template)
        .map(|capture| (capture.start(), capture.as_str()))
        .collect();


    println!("{:?}", captures);
    let mut token_position_list = Vec::new();
    let mut read_index = 0;
    let read_end = template.len();

    for (first_start, first_name) in captures {
        let first_name = first_name;
        let first_start = first_start;
        let first_end = first_start + first_name.len();

        if read_index==0 {
            let text = &template[read_index..first_start];
            println!("first_text = {:?}", text);
        }
        read_index = first_end;

        let node_position = NodePosition::new(first_name,first_start,first_end);

        if first_name == "#end" {
            if let Some(position) = stack.pop() {
                log::debug!("start:{:?}   end:{:?}",position, node_position);
                let token_position= TokenPosition::build(&position,&node_position);
                token_position_list.push(token_position);
            } else {
                // 如果没有找到匹配的开始标签，说明不匹配，报错
                return Err(format!("Unmatched #end at index {}", first_name));
            }
        }else if first_name == "#else" {
            if let Some(position) = stack.pop() {
                log::debug!("start:{:?}   end:{:?}",position, node_position);
                let token_position= TokenPosition::build(&position,&node_position);
                token_position_list.push(token_position);
                stack.push(node_position);
            }
        }else if first_name == "#elseif" {
            if let Some(position) = stack.pop() {
                log::debug!("start:{:?}   end:{:?}",position, node_position);
                let token_position= TokenPosition::build(&position,&node_position);
                token_position_list.push(token_position);
                stack.push(node_position);
            }
        }else if first_name =="#set" {
            let set_end = template[first_end..].find(')').map(|pos| first_end + pos);
            if let Some(end_pos) = set_end {
                let position = NodePosition::new("#set",first_end+1,end_pos+1);
                log::debug!("start:{:?}   end:{:?}", node_position,position);
                let token_position= TokenPosition::build(&node_position,&position);
                token_position_list.push(token_position);
            }
        }else {
            stack.push(node_position);
        }
    }

    println!("token_position_list: {:#?}", token_position_list);

    token_position_list.sort_by_key(|token| token.first_start);

    println!("token_position_list: {:#?}", token_position_list);

    Ok(tokens)
}



