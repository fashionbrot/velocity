use std::collections::VecDeque;
use std::str::FromStr;
use regex::Regex;

#[derive(Debug, PartialEq)]
enum Token {
    Number(f64),
    Operator(String),
    Parenthesis(char),
    Boolean(bool),
}

pub fn evaluate_expression(expression: &str) -> Result<bool, String> {
    let tokens = tokenize(expression)?;
    println!("tokens: {:#?}", tokens);
    let postfix = infix_to_postfix(tokens)?;
    println!("postfix: {:#?}", postfix);
    evaluate_postfix(postfix)
}

fn tokenize(expression: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();

    // 修改正则表达式，增加对 true 和 false 的支持
    let re = Regex::new(r"(\d+(\.\d+)?)|([<>=!]+|&&|\|\|)|([()])|(\btrue\b|\bfalse\b)").unwrap();

    for cap in re.captures_iter(expression) {
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
        }
    }

    Ok(tokens)
}

// fn tokenize(input: &str) -> Result<Vec<Token>, String> {
//     let mut tokens = Vec::new();
//     let mut chars = input.chars().peekable(); // 使用 peekable 以便可以看下一个字符
//
//     while let Some(c) = chars.next() {
//         match c {
//             ' ' | '\t' | '\n' => {
//                 // 跳过空白字符
//                 continue;
//             }
//             '(' | ')' => {
//                 // 处理括号
//                 tokens.push(Token::Parenthesis(c));
//             }
//             '0'..='9' | '.' => {
//                 // 处理数字
//                 let mut num_str = c.to_string();
//                 while let Some(&next_c) = chars.peek() {
//                     if next_c.is_digit(10) || next_c == '.' {
//                         num_str.push(chars.next().unwrap());
//                     } else {
//                         break;
//                     }
//                 }
//                 let num = f64::from_str(&num_str).map_err(|e| e.to_string())?;
//                 tokens.push(Token::Number(num));
//             }
//             '+' | '-' | '*' | '/' | '>' | '<' | '=' | '&' | '|' => {
//                 // 处理操作符
//                 let mut op = c.to_string();
//                 if let Some(&next_c) = chars.peek() {
//                     if (c == '+' || c == '-' || c == '=') && next_c == '=' {
//                         op.push(chars.next().unwrap());
//                     } else if (c == '&' || c == '|') && next_c == c {
//                         op.push(chars.next().unwrap());
//                     }
//                 }
//                 tokens.push(Token::Operator(op));
//             }
//             't' | 'f' => {
//                 // 处理布尔值 "true" 和 "false"
//                 let boolean_str: String = std::iter::once(c).chain(chars.by_ref().take(3)).collect();
//                 if boolean_str == "true" {
//                     tokens.push(Token::Boolean(true));
//                 } else if boolean_str == "false" {
//                     tokens.push(Token::Boolean(false));
//                 } else {
//                     return Err(format!("Unexpected boolean value: {}", boolean_str));
//                 }
//             }
//             _ => {
//                 return Err(format!("Unexpected character: {}", c));
//             }
//         }
//     }
//     Ok(tokens)
// }
//

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

    #[test]
    fn main() {
        let expression = "(1<2) && (2>1)) && true && true";
        match evaluate_expression(expression) {
            Ok(result) => println!("Result: {}", result),  // true
            Err(e) => println!("Error: {}", e),
        }
    }
}

