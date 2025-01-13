use std::collections::HashMap;
use std::f32::consts::E;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use log::log;
use meval::tokenizer::Token;
use regex::{escape, Regex};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use serde_json::Value::String as JsonString;
use crate::expression::expression_evaluator;
use crate::parse::{foreach_parse, if_parse, set_parse, text_parse};

#[derive(Debug,Clone)]
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

#[derive(Debug,Clone)]
pub enum IfBranch {
    If {
        condition: String,
        children: Option<Vec<Tokenizer>>
    }
}


impl IfBranch {
    pub fn new(condition: String, children: Vec<Tokenizer>) -> IfBranch {
        IfBranch::If { condition, children: Some(children) }
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

    pub fn new_foreach(element: &str, collection: &str, children: Vec<Tokenizer>) -> Self {
        Tokenizer::Foreach {
            element:element.to_string(),
            collection:collection.to_string(),
            children:Some(children),
        }
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


lazy_static! {
     pub static ref TAGS: Vec<&'static str> = {
        let mut tags = Vec::new();
        tags.push("#if");
        tags.push("#elseif");
        tags.push("#else");
        tags.push("#foreach");
        tags.push("#set");
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

    // 创建一个静态的 Mutex 包裹的 HashMap
    static ref TOKEN_CACHE: Arc<Mutex<HashMap<String, Vec<Tokenizer>>>> = Arc::new(Mutex::new(HashMap::new()));


}



// set 方法：将键值对插入到静态的 HashMap 中
fn set(key: String, value: Vec<Tokenizer>) {
    let mut map = TOKEN_CACHE.lock().unwrap();
    map.insert(key, value);
}

// get 方法：从静态的 HashMap 中获取值
fn get(key: &str) -> Option<Vec<Tokenizer>> {
    let map = TOKEN_CACHE.lock().unwrap();
    map.get(key).cloned()
}


pub fn get_tokens(template:&str) ->  Result<Vec<Tokenizer>,String>{

    let md5 = md5::compute(&template);
    let key = format!("{:x}", md5);

    let mut token_list = Vec::new();
    if let Some(tokens) = get(&key) {
        token_list = tokens;
    }

    if !token_list.is_empty() {
        return Ok(token_list);
    }

    match parse_position(&template,0) {
        Ok(token_position_list)=>{
            match  position_to_tokenizer(template,&token_position_list) {
                Ok(tokens)=>{
                    set(key, tokens.clone());
                    Ok(tokens)
                },
                Err(e)=>{
                    Err(e)
                }
            }
        },
        Err(e)=>{
            Err(e)
        }
    }
}



pub fn parse_position(template:&str,read_start_index:usize) -> Result<Vec<TokenPosition>,String>{

    // 生成开始结束标签
    let mut stack: Vec<NodePosition> = Vec::new(); // 用来存储开始标签的索引
    let mut captures: Vec<(usize, &str)> = TAGS_PATTERN.find_iter(template)
        .map(|capture| (capture.start(), capture.as_str()))
        .collect();

    // println!("{:?}", captures);
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
            let last_text = &template[first_start..];
            let last_start_option = find_tag_end(last_text,first_name);
            let mut last_start = 0;
            let mut last_end = 0;
            if let Some(start) = last_start_option{
                last_start = first_start+start;
                last_end = last_start+1;
            }

            let position = NodePosition::new("#set",last_start,last_end);
            log::debug!("set----start:{:?}   end:{:?}", node_position,position);
            let token_position= TokenPosition::build(&node_position,&position);
            token_position_list.push(token_position);


            // let set_end = template[first_end..].find(')').map(|pos| first_end + pos);
            // if let Some(end_pos) = set_end {
            //     let position = NodePosition::new("#set",first_end+1,end_pos+1);
            //     log::debug!("start:{:?}   end:{:?}", node_position,position);
            //     let token_position= TokenPosition::build(&node_position,&position);
            //     token_position_list.push(token_position);
            // }
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
            read_index = last_end;
        }
    }

    if read_index<read_end{

        let text_position = TokenPosition::new_text(read_index+read_start_index,read_end+read_start_index);
        log::debug!("---------- text_position:{:?}",text_position);
        position_list.push(text_position);
    }

    log::debug!("position_list: {:#?}", position_list);
    Ok(position_list)
}

pub fn position_to_tokenizer(template:&str,position_list:& [TokenPosition])-> Result<Vec<Tokenizer>,String>{

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
            let text =&template[first_start..last_end];
            let text_token = Tokenizer::new_text(text);
            tokens.push(text_token);

        }else if first_name == "#set" {

            let set_text = &template[first_end + 1..last_end - 1];
            if !set_text.is_empty() {
                if let Some((key, value)) = set_text.split_once('=') {
                    let text = Tokenizer::new_set(key.trim(), value.trim());
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

                let if_token_result =  parse_if(template,&temp);
                if if_token_result.is_err() {
                    return Err(if_token_result.unwrap_err());
                }
                let if_branch = if_token_result.unwrap();
                if_tokens.push(if_branch);

                if last_name == "#end" {
                    break;
                }

                let else_position =position_list.iter()
                    .find(|t|  if_last_name == t.first_name && if_last_start == t.first_start)
                    .cloned();
                if let Some(token_position) = else_position {
                    if_last_name = token_position.last_name.clone();
                    if_last_start = token_position.last_start;
                    temp = token_position.clone();
                }else{
                    break;
                }

            }

            tokens.push(Tokenizer::new_if(if_tokens));
        }else if first_name=="#foreach" {
            let foreach_all_text = &template[first_start..last_end];

            let mut expression_start = 0;
            let mut expression_end = 0;
            let bracket_range = find_tag_bracket_range(foreach_all_text,first_name);
            if let Some((start,end)) = bracket_range {
                expression_start = first_start+start;
                expression_end   = first_start+end;
            }else{
                return Err(format!("Error: No valid end found for the expression following the tag '{}' in the input string.",first_name))
            }
            let foreach_expression = &template[expression_start + 1..expression_end];
            let foreach_child_text = &template[expression_end + 1..last_start];


            let mut token_position_list = Vec::new();
            let token_position_result = parse_position(&foreach_child_text,0);
            if let Ok(token_position) = token_position_result {
                token_position_list.extend(token_position);
            }
            let children_tokens_result = position_to_tokenizer(&foreach_child_text, &mut token_position_list);
            if children_tokens_result.is_err() {
                return children_tokens_result;
            }
            let children_tokens = children_tokens_result.unwrap();


            // 按照 "in" 分割
            let parts: Vec<&str> = foreach_expression.split(" in ").map(str::trim).collect();
            if parts.len() == 2 {
                let variable = parts[0];
                let collection = parts[1];
                // println!("Variable: {}", variable);
                // println!("Collection: {}", collection);

                let foreach_text = &template[first_start..last_start];
                let foreach_expression_end = foreach_text.find(")");
                let mut foreach_child_text = "";
                let mut foreach_child_text_start = 0;

                if let Some(child_start) = foreach_expression_end {
                    foreach_child_text_start = first_end + child_start + 1;
                    // log::debug!("start:{} end:{}",foreach_child_text_start,last_start);
                    foreach_child_text = &template[foreach_child_text_start..last_start];

                    // log::debug!("foreach  foreach_child_text_start：{} last_start：{} foreach_child_text：{:?}",foreach_child_text_start,last_start,foreach_child_text);
                } else {
                    return Err("foreach Syntax error".to_string())
                }


                let foreach_token = Tokenizer::new_foreach(variable, collection, children_tokens);
                tokens.push(foreach_token);

            } else {
                return Err("foreach Syntax error".to_string())
            }


        }
        position_list_temp.push(position.clone());
    }

    log::debug!("tokens--------------------------\n{:#?}",tokens);

    Ok(tokens)
}


pub fn parse_if(template:&str, position:&TokenPosition) -> Result<IfBranch,String> {

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

        let children_tokens_result = position_to_tokenizer(child_text, &mut token_position_list);
        if children_tokens_result.is_err() {
            return Err(children_tokens_result.unwrap_err());
        }
        let children_tokens = children_tokens_result.unwrap();

        log::debug!("children tokens:{:?}",children_tokens);
        let if_token = IfBranch::new("true".to_string(),children_tokens);
        return Ok(if_token);

    }else if first_name=="#if" || first_name=="#elseif" {


        let text = &template[first_start..last_start];
        let mut expression_start = 0;
        let mut expression_end = 0;
        let bracket_range = find_tag_bracket_range(text,first_name);
        if let Some((start,end)) = bracket_range {
            expression_start = first_start+start;
            expression_end   = first_start+end;
        }else{
            return Err(format!("Error: No valid end found for the expression following the tag '{}' in the input string.",first_name))
        }

        log::debug!("text:{:?} expression_start:{} expression_end:{}",text,expression_start,expression_end);
        let condition = &template[expression_start+1..expression_end].trim();
        let child_text = &template[expression_end+1..last_start];


        let mut token_position_list = Vec::new();
        let token_position_result = parse_position(&child_text,0);
        if let Ok(token_position) = token_position_result {
            token_position_list.extend(token_position);
        }

        let children_tokens_result = position_to_tokenizer(child_text, &mut token_position_list);
        if children_tokens_result.is_err() {
            return Err(children_tokens_result.unwrap_err());
        }
        let children_tokens = children_tokens_result.unwrap();
        log::debug!("children_tokens:{:#?}",children_tokens);

        let if_token = IfBranch::new(condition.to_string(),children_tokens);
        return Ok(if_token);
    }

    Err("Unknown token".to_string())
}


pub fn parse_tokens(tokens:&[Tokenizer], content: &mut HashMap<String, Value>) -> Option<String> {
    if tokens.is_empty() {
        return None;
    }

    let mut output = String::new();
    for token in tokens {
        let v = parse_token(token,content);
        if let Some(v) = v {
            output.push_str(&v);
        }
    }

    Some(output)
}


pub fn parse_token(token:&Tokenizer,content: &mut HashMap<String, Value>) -> Option<String>{
    match token {
        Tokenizer::Text { .. } => {
            text_parse::text_parse(&token, content)
        }
        Tokenizer::Set { .. } => {
            set_parse::set_parse(token,content);
            None
        }
        Tokenizer::If { ..} => {
            if_parse::if_parse(token,content)
        }
        Tokenizer::Foreach { .. } => {
            foreach_parse::foreach_parse(token,content)
        }
    }
}

pub fn find_tag_bracket_range(input: &str, tag: &str) -> Option<(usize, usize)> {
    let start_index = match input.find(tag) {
        Some(index) => index,
        None => return None,
    };

    let mut stack = 0;
    let mut first_open_paren_index = None;

    for (i, c) in input[start_index..].char_indices() {
        match c {
            '(' => {
                if first_open_paren_index.is_none() {
                    first_open_paren_index = Some(start_index + i);
                }
                stack += 1;
            }
            ')' => {
                stack -= 1;
                if stack == 0 {
                    return Some((first_open_paren_index?, start_index + i));
                }
            }
            _ => {}
        }
    }
    None
}



fn find_tag_end(input: &str, tag: &str) -> Option<usize> {
    let start_index = match input.find(tag) {
        Some(index) => index,
        None => return None,
    };
    let mut stack = 0;
    for (i, c) in input[start_index..].char_indices() {
        match c {
            '(' => stack += 1,
            ')' => {
                stack -= 1;
                if stack == 0 {
                    return Some(start_index + i);
                }
            }
            _ => {}
        }
    }
    None
}


