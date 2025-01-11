use log::log;
use meval::tokenizer::Token;
use regex::Regex;
use serde::{Deserialize, Serialize};
use crate::node::{ PositionTree, TAGS_PATTERN};
use crate::node::text_node::new_node;

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
        children: Option<Vec<Tokenizer>>
    }
}

#[derive(Debug)]
pub enum IfBranch {
    If {
        condition: String,
        children: Option<Vec<Tokenizer>>
    }
}


impl IfBranch {
    pub fn new(condition: String, children: Option<Vec<Tokenizer>>) -> IfBranch {
        IfBranch::If { condition, children }
    }
}

impl Tokenizer {
    pub fn new_text(text: &str) -> Self {
        Tokenizer::Text{text:text.to_string()}
    }

    pub fn new_set(key: &str, value: &str) -> Self {
        Tokenizer::Set { key:key.to_string(), value:value.to_string() }
    }

    pub fn new_if(branches: Vec<IfBranch>) -> Self {
        Tokenizer::If{branches}
    }

    pub fn new_foreach(element: &str, collection: &str, children: Option<Vec<Tokenizer>>) -> Self {
        Tokenizer::Foreach {
            element:element.to_string(),
            collection:collection.to_string(),
            children,
        }
    }
}


#[derive(Debug)]
pub enum Position{
    Text{
        start:usize,
        end:usize
    },
    Set{
        start:usize,
        end:usize
    },
    If{
        branches: Vec<IfPosition>,
    },
    Foreach{
        first_name: String,
        first_start:usize,
        first_end :usize,
        last_name:String,
        last_start:usize,
        last_end:usize,
        element: String,
        collection: String,
        children: Vec<Position>
    }

}

#[derive(Debug)]
pub enum IfPosition{
    If{
        first_name: String,
        first_start:usize,
        first_end :usize,
        last_name:String,
        last_start:usize,
        last_end:usize,
        children: Vec<Position>
    }
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
    pub fn new_text(start:usize,end:usize) -> Self {
        // log::debug!("new_text start:{} end:{}",start,end);
        TokenPosition{
            first_name:"#text".to_string(),
            first_start:start,
            first_end:start,
            last_name:"#text".to_string(),
            last_start:end,
            last_end:end,
        }
    }
    pub fn is_root(&self, tags: &[TokenPosition]) -> bool {
        // if self.first_name =="#else" || self.first_name=="#elseif" {
        //     return false;
        // }
        let start = self.first_start;
        let end = self.last_start ;
        for x in tags {
            if x.first_start < start && start < x.last_start{
                return false
            }
            if x.first_start <end && end < x.last_start {
                return false
            }
        }
        true
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



pub fn parse_position(template:&str,read_start_index:usize) -> Result<Vec<TokenPosition>,String>{

    // 生成开始结束标签
    let mut stack: Vec<NodePosition> = Vec::new(); // 用来存储开始标签的索引
    let mut captures: Vec<(usize, &str)> = TAGS_PATTERN.find_iter(template)
        .map(|capture| (capture.start(), capture.as_str()))
        .collect();

    println!("{:?}", captures);
    let mut token_position_list = Vec::new();

    for (first_start, first_name) in captures {
        let first_name = first_name;
        let first_start = first_start;
        let first_end = first_start + first_name.len();

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

    log::debug!("token_position_list: {:#?}", token_position_list);

    let mut read_index = 0;
    let read_end = template.len();
    let mut  position_list =Vec::new();
    // let mut position_list_temp:Vec<TokenPosition> = Vec::new();

    for position in &token_position_list {
        let first_start = position.first_start;
        let first_end = position.first_end;
        let last_start = position.last_start;
        let last_end = position.last_end;
        let first_name = &position.first_name;
        let last_name = &position.last_name;
        log::debug!("---------- position first_name:{} last_name:{} first_start:{} last_end:{}",first_name,last_name,first_start,last_end);


        if position.is_root(&token_position_list) {
            if read_index < first_start  {
                position_list.push(TokenPosition::new_text(read_index,first_start));
            }
            position_list.push(position.clone());
        }
        read_index = last_end;
    }

    if read_index<read_end{

        let text_position = TokenPosition::new_text(read_index+read_start_index,read_end+read_start_index);
        log::debug!("---------- text_position:{:?}",text_position);
        position_list.push(text_position);
    }

    Ok(position_list)
}

pub fn position_to_tokenizer(template:&str,position_list:& [TokenPosition])-> Option<Vec<Tokenizer>>{
    let mut tokens:Vec<Tokenizer> = Vec::new();


    let read_start = 0;
    let read_end = template.len();

    let mut tokens:Vec<Tokenizer> = Vec::new();
    let mut position_list_temp:Vec<TokenPosition> = Vec::new();

    for position in position_list {
        let first_name = &position.first_name;
        let last_name = &position.last_name;
        let first_start = position.first_start;
        let first_end = position.first_end;
        let last_start = position.last_start;
        let last_end = position.last_end;

        log::debug!(" first_name:{} last_name:{}  first_start:{} first_end:{} last_start:{} last_end:{}",first_name,last_name,first_start,first_end,last_start,last_end);

        if first_name == "#text" {

            log::debug!("text--first_start:{:?}   last_end:{:?} first_name:{:?} last_name:{:?}",first_start,last_start,first_name,last_name);
            let text = Tokenizer::new_text(&template[first_start..last_end]);
            tokens.push(text);

        }else if first_name == "#set" {
            let set_text = &template[first_end + 1..last_end - 1];
            if !set_text.is_empty() {
                if let Some((key, value)) = set_text.split_once('=') {
                    let text = Tokenizer::new_set(key, value);
                    tokens.push(text);
                } else {
                    //TODO
                }
            } else {
                //TODO
            }
        }else if first_name == "#if" {

            let mut if_tokens = Vec::new();

            let mut if_last_name = last_name.clone();
            let mut if_last_start = last_start;

            let mut temp = position.clone();
            loop {


                let if_token =  parse_if(template,&temp);
                if let Some(token) = if_token {
                    if_tokens.push(token);
                }else{
                    break;
                }

                if last_name == "#end" {
                    break;
                }
                let else_position =position_list.iter()
                    .find(|t|  if_last_name == t.first_name && if_last_start == t.first_start)
                    .cloned();

                log::debug!("current:{:?}",temp);
                log::debug!("else_position：{:?}",else_position);

                if let Some(token_position) = else_position {
                    if_last_name = token_position.last_name.clone();
                    if_last_start = token_position.last_start;
                    temp = token_position.clone();
                }else{
                    break;
                }

            }
            tokens.push(Tokenizer::If {branches:if_tokens});
        }else if first_name=="#foreach" {

            let foreach_all_text = &template[first_start..last_end];
            let foreach_expression_end = foreach_all_text.find(")");
            let mut foreach_child_text = "";
            let mut foreach_child_text_start = 0;

            if let Some(child_start) = foreach_expression_end {

                foreach_child_text_start = first_start+child_start+1;
                log::debug!("start:{} end:{}",foreach_child_text_start,last_start);
                foreach_child_text = &template[foreach_child_text_start..last_start];

                log::debug!("foreach  foreach_child_text_start：{} last_start：{} foreach_child_text：{:?}",foreach_child_text_start,last_start,foreach_child_text);
            }else{
                //TODO error
            }


            // log::debug!("foreach_text:{:?}",foreach_text);

            let mut token_position_list = Vec::new();
            let token_position_result = parse_position(&foreach_child_text,0);
            if let Ok(token_position) = token_position_result {
                token_position_list.extend(token_position);
            }
            let children_tokens = position_to_tokenizer(&foreach_child_text, &mut token_position_list);

            let re = Regex::new(r"#foreach\(\s*(.*?)\s*\)").unwrap();
            if let Some(captures) = re.captures(foreach_all_text) {
                if let Some(condition) = captures.get(1) {
                    let condition_str = condition.as_str();
                    // 按照 "in" 分割
                    let parts: Vec<&str> = condition_str.split(" in ").map(str::trim).collect();

                    if parts.len() == 2 {
                        let variable = parts[0];
                        let collection = parts[1];
                        println!("Variable: {}", variable);
                        println!("Collection: {}", collection);

                        let foreach_text = &template[first_start..last_start];
                        let foreach_expression_end = foreach_text.find(")");
                        let mut foreach_child_text = "";
                        let mut foreach_child_text_start = 0;

                        if let Some(child_start) = foreach_expression_end {
                            foreach_child_text_start = first_end + child_start + 1;
                            log::debug!("start:{} end:{}",foreach_child_text_start,last_start);
                            foreach_child_text = &template[foreach_child_text_start..last_start];

                            log::debug!("foreach  foreach_child_text_start：{} last_start：{} foreach_child_text：{:?}",foreach_child_text_start,last_start,foreach_child_text);
                        } else {
                            //TODO error
                        }


                        let foreach_token = Tokenizer::new_foreach(variable, collection, children_tokens);

                        tokens.push(foreach_token);
                    } else {
                        println!("Invalid condition format.");
                    }
                }
            }
        }
        position_list_temp.push(position.clone());
    }

    Some(tokens)
}

pub fn get_branch(tags: &[PositionTree], current_end: usize) -> Option<&PositionTree> {
    tags.iter()
        .find(|tag| tag.start == current_end)
}


pub fn parse_if(template:&str, position:&TokenPosition) -> Option<IfBranch> {

    let first_name = &position.first_name;
    let last_name = &position.last_name;
    let first_start = position.first_start;
    let first_end = position.first_end;
    let last_start = position.last_start;
    let last_end = position.last_end;
    log::debug!("template:{:?} first_end：{} last_start：{}",template,first_end,last_start);

    let child_text = &template[first_end..last_start];
    log::debug!("child_text:{:?}",child_text);

    if first_name =="#else" {

        let mut token_position_list = Vec::new();
        let token_position_result = parse_position(&child_text,0);
        if let Ok(token_position) = token_position_result {
            token_position_list.extend(token_position);
        }

        let children_tokens = position_to_tokenizer(child_text, &mut token_position_list);

        log::debug!("children tokens:{:?}",children_tokens);
        let if_token = IfBranch::new("true".to_string(),children_tokens);
        return Some(if_token);

    }else if first_name=="#if" || first_name=="#elseif" {

        let if_child_start = child_text.find(")");

        let mut if_child_text = "";
        let mut child_start = 0;
        if let Some(if_child_start) = if_child_start {
            child_start = first_end+if_child_start+1;
            log::debug!("start:{} end:{}",(first_end+if_child_start+1),last_start);
            if_child_text = &template[first_end+if_child_start+1..last_start];
        }else{
            //TODO error
        }


        let mut token_position_list = Vec::new();
        let token_position_result = parse_position(&if_child_text,0);
        if let Ok(token_position) = token_position_result {
            token_position_list.extend(token_position);
        }

        let children_tokens = position_to_tokenizer(if_child_text, &mut token_position_list);
        log::debug!("children_tokens:{:#?}",children_tokens);

        let text = &template[first_start..last_start];
        let mut re = Regex::new(r"(?s)#if\s*\(\s*(.*?)\s*\)").unwrap();
        if first_name=="#elseif" {
            re = Regex::new(r"(?s)#elseif\s*\(\s*(.*?)\s*\)").unwrap();
        }
        if let Some(captures) = re.captures(text) {
            if let Some(condition) = captures.get(1) {
                log::debug!("-------------condition:{:?}",condition.as_str());
                let if_token = IfBranch::new(condition.as_str().to_string(),children_tokens);
                return Some(if_token);
            }
        }
    }

    None
}
// pub fn get_branch(tags: &[PositionTree], current_end: usize) -> Option<&PositionTree> {
//     tags.iter()
//         .find(|tag| tag.start == current_end)
// }

pub fn parse_token(template:&str)->Result<Vec<Tokenizer>,String>{
    log::debug!("parse_token start");
    if template.is_empty() {
        return Err("template empty".to_string()); ;
    }
    let mut tokens:Vec<Tokenizer> = Vec::new();

    let position_list_result = parse_position(&template,0);
    if let Err(err) = position_list_result {
        return Err(err);
    }

    let token_position_list = position_list_result?;





    let read_start = 0;
    let read_end = template.len();

    let mut token_tree:Vec<TokenPosition> = Vec::new();

    // for position in token_position_list {
    //     let first_start = position.first_start;
    //     let first_end = position.first_end;
    //     let last_start = position.last_start;
    //     let last_end = position.last_end;
    //
    //
    //     let children_list:Vec<PositionTree> = token_tree_temp
    //         .iter()
    //         .filter(|t|  first_start< t.start && last_end> t.end)
    //         .cloned()
    //         .collect();
    //
    //     if !children_list.is_empty() {
    //         children_list.iter().for_each(|child| {
    //             if let Some(index) = token_tree_temp.iter().position(|x| x.start == child.start && x.end == child.end) {
    //                 token_tree_temp.swap_remove(index);
    //             }
    //         });
    //
    //     }
    //
    //
    //
    // }


    Ok(tokens)
}


pub fn template_token(read_start:usize,template:&str,position: TokenPosition,token_position_list:&Vec<TokenPosition>) ->  Result<Vec<Tokenizer>,String>{
    let mut token_list =Vec::new();
    let read_end = template.len();

    let first_start = position.first_start;
    let first_end = position.first_end;
    let last_start = position.last_start;
    let last_end = position.last_end;

    if read_start<first_start {
        let text = &template[read_start..first_start];
        token_list.push(Tokenizer::new_text(text));
    }


    let children_list:Vec<TokenPosition> = token_position_list
        .iter()
        .filter(|t|  first_start< t.first_start && last_end> t.first_end)
        .cloned()
        .collect();





    Ok(token_list)
}



