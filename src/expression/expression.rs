use lazy_static::lazy_static;
use regex::{escape, Regex};
use crate::expression::expr_eval;

#[derive(Debug, PartialEq)]
enum Token {
    Condition{
      text: String,
    },
    Connector{
        text: String,
    },
    Bracket{
        branch:Vec<Token>,
    }
}

lazy_static!(
         pub static ref TAGS: Vec<&'static str> = {
        let mut tags = Vec::new();
        tags.push("&&");
        tags.push("||");
        tags.push("(");
        tags.push(")");
        // tags.push("<");
        // tags.push(">");
        // tags.push(">=");
        // tags.push("<=");
        // tags.push("==");
        // tags.push("!=");
                // tags.push("+");
                // tags.push("-");
                // tags.push("*");
                // tags.push("/");
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
    let mut read_start = 0;
    let read_end = input.len();
    for (tag_index,capture) in TAGS_PATTERN.find_iter(&input).enumerate() {
        let first_start = capture.start();
        let mut first_end = capture.end();
        let tag = capture.as_str();

        if read_start > first_start {
            continue;
        }
        println!("index:{} capture:{:?}",tag_index, capture);
        // if tag == "&&" || tag == "||" {
        //     let first_text = &input[read_start..first_start].trim();
        //     if !first_text.is_empty() {
        //         println!("--------------------------------------first_text:{:?}",first_text);
        //         tokens.push(Token::Condition{text: first_text.to_string()});
        //     }
        // }
        let first_text = &input[read_start..first_start].trim();
        if !first_text.is_empty() {
            println!("--------------------------------------first_text:{:?}",first_text);
            tokens.push(Token::Condition{text: first_text.to_string()});
        }

        if tag == "&&" {
            tokens.push(Token::Connector {text:"&&".to_string()});
        }else if tag == "||" {
            tokens.push(Token::Connector {text:"||".to_string()});
        }else if tag == "(" {

            let right_bracket_index_option = find_matching_parenthesis(input,first_start);
            println!("{:?}", right_bracket_index_option);
            if let Some(right_bracket_index) = right_bracket_index_option {

                let inner_text = &input[first_end..right_bracket_index];


                println!("()---------------------------start:{} end:{} text:{:?}",read_start,first_end,inner_text);

                let child_result = parse_position(inner_text);
                if let Ok(child) = child_result {
                    tokens.push(Token::Bracket{branch:child});
                }

                first_end = right_bracket_index+1;
            }
        }else{
            tokens.push(Token::Connector {text:tag.to_string()});
        }
        read_start = first_end;
    }

    if read_start < read_end{
        let last_text = &input[read_start..read_end].trim();
        if !last_text.is_empty() {
            tokens.push(Token::Condition{text: last_text.to_string()});
        }
    }

    println!("tokens------------{:#?}", tokens);
    println!("{:?}", input);

    Ok(tokens)
}

fn find_matching_parenthesis(input: &str, start_index:usize) -> Option<usize> {
    println!("---------------------------------------------------------------input:{:?} start_index:{}",input,start_index);
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


pub fn eval_tokens(tokens:Vec<Token>) -> Result<bool,String> {
    if tokens.len() == 0 {
        return Ok(false);
    }

    let mut output = String::new();
    for token in tokens {
        match token {
            Token::Condition { text } => {
                if text == "false" {
                    output.push_str("false");
                }else if text == "true" {
                    output.push_str("true");
                }else if text == "!false" {
                    output.push_str("true");
                }else if text == "!true" {
                    output.push_str("false");
                }else{
                    output.push_str(format!("{}",group_condition(text.clone())).as_str());
                }
            }
            Token::Connector { text } => {
                output.push_str(&text);
            }
            Token::Bracket { branch } => {
                if let Ok(result) = eval_tokens(branch) {
                    output.push_str(format!("{}",result).as_str());
                }
            }
        }
    }

    println!("output---------{}", output);

    Ok(eval_expression_based_on_rules(output.as_str()))
}

pub fn group_condition(condition: String) -> bool {

    // 查找操作符
    if let Some(mat) = CONDITION_REGEX.find(&condition) {
        let mat = mat.as_str();
        let m = condition.split(mat).collect::<Vec<&str>>();
        // 提取操作符前的部分作为 left
        let left = m[0].to_string();
        // 提取操作符后的部分作为 right
        let mut right = m[1].to_string();

        if contains_meval(right.as_str()) {
            right = meval_eval_str(right);
        }

        println!("-----------------------------------left:{:?} right:{:?} mat:{}",left, right,mat);

        eval_condition(left, right, mat.to_string())
    }else{
        false
    }
}

pub fn meval_eval_str(input:String)->String{
    // if let Ok(new_value) = expr_eval::eval(input.as_str()) {
    //     return new_value.to_string();
    // }
    input.to_string()
}

pub fn eval_condition(left: String, right: String, connector: String) -> bool {
    println!("eval_condition-----{} {} {}", left, right, connector);
    match connector.as_str() {
        "==" => left == right,
        "!=" => left != right,
        "<" | ">" | "<=" | ">=" => {
            // 数值比较运算符，尝试转换为数字
            let left_num = left.parse::<f64>();
            let right_num = right.parse::<f64>();
            match (left_num, right_num) {
                (Ok(l), Ok(r)) => match connector.as_str() {
                    "<" => l < r,
                    ">" => l > r,
                    "<=" => l <= r,
                    ">=" => l >= r,
                    _ => false, // 不可能到达这里
                },
                _ => {
                    // 如果无法转换为数字，则直接返回 false
                    eprintln!(
                        "Cannot perform numeric comparison on non-numeric values: '{}' and '{}'",
                        left, right
                    );
                    false
                }
            }
        }
        _ => {
            eprintln!("Unsupported connector: {}", connector);
            false
        }
    }
}

lazy_static!(

    static ref  MEVAL_REGEX:Regex = Regex::new(r"(\+|\-|\*|\/|\%|\^|&&|\|\||==|!=|sqrt|abs|exp|ln|sin|cos|tan|asin|acos|atan|atan2|sinh|cosh|tanh|asinh|acosh|atanh|floor|ceil|round|signum|max|min|pi|e)").unwrap();
    // 定义正则表达式，匹配操作符
    static ref  CONDITION_REGEX:Regex = Regex::new(r"(>=|<=|==|!=|>|<)").unwrap();
);

fn contains_meval(input: &str) -> bool {
    // 正则表达式匹配数学运算符、逻辑运算符、函数和常量
    // 检查输入字符串是否包含匹配的内容
    MEVAL_REGEX.is_match(input)
}

pub fn eval_expression_based_on_rules(expression: &str) -> bool {
    // 规则 1：如果字符串中包含 "|| true"，则返回 true
    if expression.contains("||true") || expression.contains("true||") {
        return true;
    }
    // 规则 2：如果字符串中没有 "|| true"，且包含 "false"，则返回 false
    if expression.contains("false") {
        return false;
    }
    // 默认返回 true（当没有 "false" 且没有 "|| true" 时）
    true
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_tokens() {
        if false && false {
            println!("--------------------------true");
        }

        // let expression = "true &&  ( 11==11 &&  22==22 ||  33==33 && (44==44  ||  (55==55)) ) || true || true  && 张三!=李四";
        let expression = "17>1+2+3*4";
        let tokens = parse_position(expression);

        println!("{:?}", tokens);

        if let Ok(token) = tokens {
            let result = eval_tokens(token).unwrap();
            println!("result:{:?}", result);
        }
    }
}

