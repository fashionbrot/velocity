use std::string::String;

#[derive(Debug, PartialEq)]
enum Token {
    Condition {
        text: String,
    },
    Connector {
        text: String,
    },
    Bracket {
        branch: Vec<Token>,
    },
}

fn evaluate_expression(tokens: &[Token]) -> Result<f64, String> {
    // 用于存储操作数和运算符
    let mut values = Vec::new();
    let mut operators:Vec<String> = Vec::new();

    // 定义运算优先级
    let precedence = |op: &str| match op {
        "*" | "/" => 2,
        "+" | "-" => 1,
        "==" | ">" | "<" | "&&" | "||" => 0,
        _ => -1,
    };

    // 执行一次运算
    let apply_operator = |values: &mut Vec<f64>, operators: &mut Vec<String>| -> Result<(), String> {
        if let (Some(op), Some(b), Some(a)) = (operators.pop(), values.pop(), values.pop()) {
            let result = match op.as_str() {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => {
                    if b == 0.0 {
                        return Err("Division by zero".into());
                    }
                    a / b
                }
                ">" => (a > b) as i32 as f64,
                "==" => (a == b) as i32 as f64,
                "&&" => ((a != 0.0) && (b != 0.0)) as i32 as f64,
                "||" => ((a != 0.0) || (b != 0.0)) as i32 as f64,
                _ => return Err(format!("Unsupported operator: {}", op)),
            };
            values.push(result);
            Ok(())
        } else {
            Err("Invalid expression".into())
        }
    };

    let mut i = 0;
    while i < tokens.len() {
        match &tokens[i] {
            Token::Condition { text } => {
                // 将条件解析为数值
                if let Ok(value) = text.parse::<f64>() {
                    values.push(value);
                } else {
                    return Err(format!("Invalid condition: {}", text));
                }
            }
            Token::Connector { text } => {
                while let Some(op) = operators.last() {
                    if precedence(op.as_str()) >= precedence(text) {
                        apply_operator(&mut values, &mut operators)?;
                    } else {
                        break;
                    }
                }
                operators.push(text.clone());

            }
            Token::Bracket { branch } => {
                // 递归求值
                let result = evaluate_expression(branch)?;
                values.push(result);
            }
        }
        i += 1;
    }

    // 清空栈中剩余的操作符
    while !operators.is_empty() {
        apply_operator(&mut values, &mut operators)?;
    }

    // 最终结果
    if values.len() == 1 {
        Ok(values[0])
    } else {
        Err("Expression evaluation error".into())
    }
}

#[test]
fn main() {
    let tokens = vec![
        Token::Condition {
            text: "true".to_string(),
        },
        Token::Connector {
            text: "&&".to_string(),
        },
        Token::Bracket {
            branch: vec![
                Token::Condition {
                    text: "11==11".to_string(),
                },
                Token::Connector {
                    text: "&&".to_string(),
                },
                Token::Condition {
                    text: "22==22".to_string(),
                },
                Token::Connector {
                    text: "||".to_string(),
                },
                Token::Condition {
                    text: "33==33".to_string(),
                },
                Token::Connector {
                    text: "&&".to_string(),
                },
                Token::Bracket {
                    branch: vec![
                        Token::Condition {
                            text: "44==44".to_string(),
                        }
                    ],
                },
            ],
        },
        Token::Connector {
            text: "||".to_string(),
        },
        Token::Condition {
            text: "true".to_string(),
        },
        Token::Connector {
            text: "||".to_string(),
        },
        Token::Condition {
            text: "false".to_string(),
        },
        Token::Connector {
            text: "&&".to_string(),
        },
        Token::Condition {
            text: "13".to_string(),
        },
        Token::Connector {
            text: ">".to_string(),
        },
        Token::Bracket {
            branch: vec![
                Token::Condition {
                    text: "4+5*4".to_string(),
                },
            ],
        },
    ];
    match evaluate_expression(&tokens) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
