use lazy_static::lazy_static;
use regex::{escape, Regex};

#[derive(Debug, PartialEq)]
enum Token {
    Value(String),
    BinaryOperator {
        operator: String,
        left: Box<Token>,
        right: Box<Token>,
    },
    Parenthesis {
        inner: Box<Token>,
    },
    Connector(String), // 新增的连接符
}


#[derive(Debug, PartialEq)]
struct  Position{
    pub start: usize,
    pub end: usize,
    pub tag: String,
    pub text: String,
}
impl Position {
    pub fn new(start: usize, end: usize, tag: String) -> Position {
        Position{
            start,
            end,
            tag,
            text: "".to_string(),
        }
    }
    pub fn build(start: usize, end: usize, tag: String,text:String) -> Position {
        Position{
            start,
            end,
            tag,
            text: text,
        }
    }
}

lazy_static!(
         pub static ref TAGS: Vec<&'static str> = {
        let mut tags = Vec::new();
        tags.push("&&");
        tags.push("||");
        tags.push("(");
        tags.push(")");
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

);

pub fn parse_position(input:&str) ->Result<Vec<Token>,String> {
    let mut tokens = Vec::new();

    println!("{:?}", input);
    let  captures: Vec<(usize, &str)> = TAGS_PATTERN.find_iter(&input)
        .map(|capture| (capture.start(), capture.as_str()))
        .collect();
    let mut position_list = Vec::new();

    println!("{:?}", captures);

    let mut read_start = 0;
    let read_end = input.len();
    for (index,tag) in captures {
        println!("{} {}", index, tag);
        let tag = tag;
        let start = index;
        let mut end = index+tag.len();
        let position = Position::new(start,end,tag.to_string());

        if tag == "(" {
            let text = &input[start..];
            let right_bracket_index_option = find_matching_parenthesis(text,start);
            println!("{:?}", right_bracket_index_option);
            if let Some(right_bracket_index) = right_bracket_index_option {
                end = start+right_bracket_index;
                let text = &input[start+1..end];
                println!("{:?}",text);
                let build = Position::build(start,end,tag.to_string(),text.to_string());
                position_list.push(build);
            }

        }else if tag == "&&" {
            let text = &input[read_start..start];
            println!("&& -------------{}", text);
            let build = Position::build(read_start,start,tag.to_string(),text.to_string());
            position_list.push(build);
        }else if tag == "||" {
            let text = &input[read_start..start];
            println!("|| -------------{}", text);
            let build = Position::build(read_start,start,tag.to_string(),text.to_string());
            position_list.push(build);
        }
        
        read_start+=end;
    }

    println!("{:#?}", position_list);

    Ok(tokens)
}

fn find_matching_parenthesis(input: &str, start_index:usize) -> Option<usize> {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_tokens() {

            let expression = "true && (1 == 1 && ((2 == 2) || (3 == 3) || 4 == 4) || true) || false";
        let tokens = parse_position(expression);
        println!("{:?}", tokens);
    }


}

