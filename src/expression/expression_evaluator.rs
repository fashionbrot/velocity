use std::collections::VecDeque;
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
enum Token {
    Number(f64),
    Operator(String),
    Parenthesis(char),
    Boolean(bool),
    String(String)
}

lazy_static! {
    // static ref EXPRESSION_TOKEN_REGEX: Regex = Regex::new(
    //     r"(\d+(\.\d+)?)|([<>=!]+|&&|\|\|)|([()])|(\btrue\b|\bfalse\b)|\$\{[^\}]+\}"
    // ).unwrap();
    static ref EXPRESSION_TOKEN_REGEX: Regex = Regex::new(
        r#"(\d+(\.\d+)?)|([<>=!]+|&&|\|\|)|([()])|(\btrue\b|\bfalse\b)|\"[^\"]*\"|\$\{[^\}]+\}"#
    ).unwrap();
}

pub fn evaluate_expression(expression: &str) -> Result<bool, String> {
    log::debug!("Evaluating expression: {}", expression);
    let tokens = tokenize(expression);
    if tokens.is_err() {
        return Err(tokens.unwrap_err());
    }

    log::debug!("tokens: {:#?}", tokens);
    let postfix = infix_to_postfix(tokens.unwrap());
    if postfix.is_err() {
        return Err(postfix.unwrap_err());
    }
    log::debug!("postfix: {:#?}", postfix);
    evaluate_postfix(postfix.unwrap())
}
fn tokenize(expression: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();

    // 修改正则表达式，增加对 true 和 false 的支持
    // let re = Regex::new(r"(\d+(\.\d+)?)|([<>=!]+|&&|\|\|)|([()])|(\btrue\b|\bfalse\b)|\$\{[^\}]+\}").unwrap();

    for cap in EXPRESSION_TOKEN_REGEX.captures_iter(expression) {
        if let Some(num) = cap.get(1) {
            // 处理数字
            tokens.push(Token::Number(num.as_str().parse().unwrap()));
        } else if let Some(op) = cap.get(3) {
            // 处理操作符
            tokens.push(Token::Operator(op.as_str().to_string()));
        } else if let Some(paren) = cap.get(4) {
            // 处理括号
            tokens.push(Token::Parenthesis(paren.as_str().chars().next().unwrap()));
        } else if let Some(bool_str) = cap.get(5) {
            // 处理布尔值 true 和 false
            let boolean = match bool_str.as_str() {
                "true" => Token::Boolean(true),
                "false" => Token::Boolean(false),
                _ => return Err(format!("Unexpected boolean value: {}", bool_str.as_str())),
            };
            tokens.push(boolean);
        } else if let Some(var) = cap.get(6) {
            // 处理 `${}` 中的变量
            tokens.push(Token::String(var.as_str().to_string()));  // 处理字符串
        }
    }

    Ok(tokens)
}



fn infix_to_postfix(tokens: Vec<Token>) -> Result<Vec<Token>, String> {
    let mut output = Vec::new();
    let mut operators:VecDeque<String> = VecDeque::new();

    for token in tokens {
        match token {
            Token::Number(_) => output.push(token),
            Token::Operator(ref op) => {
                while let Some(top) = operators.back() {
                    if precedence(top) >= precedence(&op) {
                        output.push(Token::Operator(operators.pop_back().unwrap()));
                    } else {
                        break;
                    }
                }
                operators.push_back(op.clone()); // 使用 String 类型
            }
            Token::Parenthesis('(') => operators.push_back("(".to_string()),  // 将 "(" 转换为 String
            Token::Parenthesis(')') => {
                while let Some(top) = operators.pop_back() {
                    if top == "(" {
                        break;
                    } else {
                        output.push(Token::Operator(top));
                    }
                }
            }
            Token::Boolean(b) => {
                output.push(Token::Boolean(b));  // 处理布尔值
            }
            Token::Parenthesis(_) => {}
            Token::String(s) => {
                output.push(Token::String(s.to_string()));
            }
        }
    }

    while let Some(op) = operators.pop_back() {
        output.push(Token::Operator(op));  // 将剩余的操作符加入输出
    }

    Ok(output)
}


fn precedence(op: &str) -> usize {
    match op {
        "&&" | "||" => 1,
        ">" | "<" | "==" | ">=" | "<=" => 2,
        _ => 0,
    }
}

fn evaluate_postfix(tokens: Vec<Token>) -> Result<bool, String> {
    let mut stack = Vec::new();

    for token in tokens {
        match token {
            Token::String(s) => {
                log::debug!("{}", s);
            },
            Token::Number(n) => stack.push(n),
            Token::Boolean(b) => {
                // 将布尔值转换为数字：true -> 1.0，false -> 0.0
                stack.push(if b { 1.0 } else { 0.0 });
            }
            Token::Operator(op) => {
                let right = stack.pop().ok_or("Insufficient operands")?;
                let left = stack.pop().ok_or("Insufficient operands")?;
                let result = match op.as_str() {
                    ">" => Ok(left > right),
                    "<" => Ok(left < right),
                    "==" => Ok(left == right),
                    ">=" => Ok(left >= right),
                    "<=" => Ok(left <= right),
                    "&&" => Ok(left != 0.0 && right != 0.0),
                    "||" => Ok(left != 0.0 || right != 0.0),
                    "!=" => Ok(left != right),
                    _ => Err("Invalid operator".to_string()),
                }?;
                stack.push(if result { 1.0 } else { 0.0 });
            }
            _ => {}
        }
    }

    if let Some(result) = stack.pop() {
        Ok(result != 0.0)
    } else {
        Err("Invalid expression".to_string())
    }
}





#[cfg(test)]
mod tests {
    use crate::expression::expression_evaluator::evaluate_expression;
    use std::io::Write;

    #[test]
    fn main() {
        std::env::set_var("RUST_LOG", "debug");
        env_logger::Builder::from_default_env()
            .format(|buf, record| {
                writeln!(
                    buf,
                    "[{} - {}] - {} ",
                    record.target(),
                    record.line().unwrap_or(0),
                    record.args()
                )
            })
            .init();

        // let expression = "1==1";
        let expression= "你好==你好";
        match evaluate_expression(expression) {
            Ok(result) => println!("Result: {}", result),  // true
            Err(e) => println!("Error: {}", e),
        }
    }
}

