use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use log::log;
use meval::tokenizer::Token;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use serde_json::Value::String as JsonString;
use crate::expression::expression_evaluator;
use crate::node;
use crate::node::{ExpressionNode, PositionTree, TAGS_PATTERN};
use crate::node::text_node::{new_node};

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


pub fn get_tokens(template:&str) ->  Option<Vec<Tokenizer>>{

    let md5 = md5::compute(&template);
    let key = format!("{:x}", md5);

    let mut token_list = Vec::new();
    if let Some(tokens) = get(&key) {
        token_list = tokens;
    }

    if token_list.is_empty() {
        if let Ok(token_position_list) = parse_position(&template,0) {
            if let Some(tokens) = position_to_tokenizer(template,&token_position_list) {
                set(key, tokens.clone());
                return Some(tokens);
            }
        }
    }else{
        return Some(token_list);
    }

    None
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

            tokens.push(Tokenizer::new_if(if_tokens));
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


pub fn parse_token(tokens:&[Tokenizer],content: &mut HashMap<String, Value>) -> Option<String> {
    if tokens.is_empty() {
        return None;
    }

    let mut value = "".to_string();
    for token in tokens {
        let v = get_node_text(token,content);
        if let Some(v) = v {
            value.push_str(&v);
        }
    }

    Some(value)
}


pub fn get_node_text(token:&Tokenizer,content: &mut HashMap<String, Value>) -> Option<String>{
    match token {
        Tokenizer::Text { .. } => {
            text_parse(&token, content)
        }
        Tokenizer::Set { .. } => {
            set_parse(token,content)
        }
        Tokenizer::If { branches} => {
            if_parse(token,content)
        }
        Tokenizer::Foreach { .. } => {
            parse_foreach(token,content)
        }
    }
}


pub fn text_parse(token:&Tokenizer, context: &mut HashMap<String, Value>) -> Option<String> {

    if let Tokenizer::Text { text } = token {
        if text.is_empty() {
            return None;
        }
        let value =normalize_variable_syntax(text.as_str(),context);
        return parse_string(&value);
    }

    None

}

pub fn set_parse(token :&Tokenizer, content: &mut HashMap<String, Value>) -> Option<String>{

    if let Tokenizer::Set { key,value } = token {
        println!("key:{} vlaue:{}",key,value);
        let k = extract_variable(&key);
        if let Some(key) = k{
            if let Ok(number) = value.trim_matches('"').parse::<isize>() {
                println!("Parsed as number: key:{} value:{}", key, number);
                if let Some(value) = content.get_mut(&key) {
                    *value = Value::Number(Number::from(number));
                }else{
                    content.insert(key, Value::Number(Number::from(number)));
                }
            } else {
                println!("Parsed as string: key:{} value:{}", key, value);
                if let Some(value) = content.get_mut(&key) {
                    *value = Value::String(value.to_string());
                }else{
                    content.insert(key, Value::String(value.to_string()));
                }
            }
        }

    }

    None
}

fn extract_variable(input: &String) -> Option<String> {
    // 使用懒加载正则，避免每次调用都编译正则
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(r"^\$\{?(.*?)\}?$").unwrap();
    }

    // 尝试匹配并提取变量名
    RE.captures(&input)
        .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
}



pub fn if_parse(token:&Tokenizer, context:&mut HashMap<std::string::String, Value>) -> Option<String> {
    if let Tokenizer::If { branches} = token {
        for branch in branches {
            if let IfBranch::If{condition,children,..} = branch {

                let if_condition = normalize_variable_syntax(condition.as_str(),context);

                if let Ok(expression) = expression_evaluator::evaluate_expression(&if_condition) {
                    if expression {
                        println!("if expression:{:?}",expression);

                        if let Some(child) = children{

                            let mut output = String::new();
                            for child_token in child {

                                let result = get_node_text(child_token,context);

                                if let Some(text) = result {
                                    if let Some(value) = parse_string(&text) {
                                        output.push_str(&value);
                                    }else{
                                        output.push_str(&text);
                                    }
                                }
                            }
                            return Some(output);
                        }
                    }
                }

            }
        }
    }
    None
}



pub fn parse_foreach(token:&Tokenizer, context:&mut HashMap<String, Value>) -> Option<std::string::String> {

    if let Tokenizer::Foreach { element,collection,children } = token {
        let mut output = String::new();

        let mut element_key = element.to_string();
        if let Some(key) = extract_variable(&element_key) {
            element_key  = key.to_string();
        }
        let mut collection_key = collection.to_string();
        if let Some(key) = extract_variable(&collection_key) {
            collection_key = key.to_string();
        }


        log::debug!("parse_foreach key:{:?} element:{} collection:{}",element_key,element,collection);

        // 从 context 中获取集合对象
        if let Some(Value::Array(list)) = context.get(&collection_key) {
            // 将集合对象更新到 context 中
            // context.insert(key, Value::Array(list.clone()));
            let items = list.clone();
            // 遍历数组中的每个元素
            for item in items {
                log::debug!("Processing item: {:?}", item);

                context.insert(element_key.clone(), item);

                if let Some(child) = children{
                    for child_token in child {
                        let result = get_node_text(child_token,context);
                        if let Some(text) = result {

                            let value = normalize_variable_syntax(text.as_str(),context);

                            if let Some(value) = parse_string(&value) {
                                output.push_str(&value);
                            }else{
                                output.push_str(&value);
                            }

                        }
                    }
                }

            }
        }
        return Some(output);
    }

    None
}


lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r"\$\{ *([^}]+) *\}|\$([a-zA-Z_][a-zA-Z0-9_]*)").unwrap(); // 支持 $age 和 ${age}
}

pub fn normalize_variable_syntax(input: &str, context: &HashMap<String, Value>) -> String {
    // 使用正则表达式进行替换
    RE.replace_all(input, |caps: &regex::Captures| {
        // 检查是 ${} 形式还是 $ 形式
        let key = if let Some(key) = caps.get(1) {
            key.as_str().trim()
        } else if let Some(key) = caps.get(2) {
            key.as_str()
        } else {
            return String::new(); // 如果没有匹配到有效的变量，返回空字符串
        };

        // 查找对应的值
        match context.get(key) {
            Some(value) => match value {
                Value::String(s) => s.clone(), // 如果是 String 类型，直接返回内容，不加引号
                Value::Number(n) => n.to_string(), // 如果是数字，转换为字符串
                Value::Bool(b) => b.to_string(),   // 如果是布尔值，转换为字符串
                _ => format!("{}", value), // 其他类型，直接转换为字符串
            },
            None => format!("${{{}}}", key), // 如果没找到，返回原始变量格式
        }
    })
        .to_string()
}


pub fn parse_string(text: &String) -> Option<String> {
    if text.len()>1 {
        if text.len()==2 && text.starts_with("\r\n") {
            return None;
        }

        if is_wrapped_with_crlf(text) {
            return Some(format!("{}\n",remove_surrounding_crlf(text)));
        }else{
           return Some(text.to_string());
        }
    }
    None
}

fn is_wrapped_with_crlf(input: &str) -> bool {
    let trimmed = input
        .trim_start_matches(' ')  // 去掉开头的空格
        .trim_end_matches(' ');   // 去掉结尾的空格
    if input.matches("\r\n").count()==1 {
        return false;
    }
    // println!("is_wrapped_with_crlf trimmed:{:?}",trimmed);
    trimmed.starts_with("\r\n") && trimmed.ends_with("\r\n")
}
fn remove_surrounding_crlf(input: &str) -> String {

    if input.starts_with("\r\n") && input.ends_with("\r\n") {
        return  input[2..input.len() - 2].to_string();
    }

    let start_ = input.find("\r\n");
    let end_ = input.rfind("\r\n");
    if start_.is_none() || end_.is_none() {
        return input.to_string();
    }
    let start = start_.unwrap();
    let end = end_.unwrap();

    if input.starts_with("\r\n") {
        let m_text = &input[2..end];
        let end_text = &input[end+2..];
        return format!("{}{}", m_text, end_text)
    }else if input.ends_with("\r\n") {
        let start_text = &input[0..start];
        let m_text = &input[(start+2)..end];
        return format!("{}{}", start_text, m_text)
    }

    let start_text = &input[0..start];
    let m_text = &input[(start+2)..end];
    let end_text = &input[end+2..];

    format!("{}{}{}", start_text, m_text, end_text)
}
