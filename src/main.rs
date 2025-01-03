use tokio::io::split;
use velocity::tag::tag_parse;

fn evaluate_expression(expression: &str) -> Result<f64, String> {
    // 预处理布尔值：将 true 和 false 替换为 1 和 0
    let expression = expression
        .replace("true", "1")
        .replace("false", "0");

    // 如果包含括号，递归处理括号中的内容
    if expression.contains('(') && expression.contains(')') {
        // 处理括号内的子表达式
        let inner_expr = extract_inner_expression(&expression)?;
        println!("{}", inner_expr);
        let result = evaluate_expression(&inner_expr)?;
        return Ok(result);
    }

    // 处理逻辑运算符（&& 和 ||）
    if expression.contains("&&") {
        let parts: Vec<&str> = expression.split("&&").collect();
        if parts.len() > 1 {
            let mut results = Vec::new();
            for part in parts {
                // 计算每个子表达式
                results.push(evaluate_expression(part)? != 0.0);
            }
            // 合并结果：逻辑与运算：所有子表达式都为 true
            let result = results.iter().all(|&r| r);
            return Ok(result as i32 as f64);
        }
    } else if expression.contains("||") {
        let parts: Vec<&str> = expression.split("||").collect();
        if parts.len() > 1 {
            let mut results = Vec::new();
            for part in parts {
                // 计算每个子表达式
                results.push(evaluate_expression(part)? != 0.0);
            }
            // 合并结果：逻辑或运算：至少一个子表达式为 true
            let result = results.iter().any(|&r| r);
            return Ok(result as i32 as f64);
        }
    }



    // 如果没有逻辑运算符或括号，尝试作为比较运算或算术运算处理
    if expression.contains(">") {
        let parts: Vec<&str> = expression.split('>').collect();
        if parts.len() == 2 {
            let left = evaluate_arithmetic(parts[0])?;
            let right = evaluate_arithmetic(parts[1])?;
            return Ok((left > right) as i32 as f64); // 返回 1.0 或 0.0
        }
    } else if expression.contains("<") {
        let parts: Vec<&str> = expression.split('<').collect();
        if parts.len() == 2 {
            let left = evaluate_arithmetic(parts[0])?;
            let right = evaluate_arithmetic(parts[1])?;
            return Ok((left < right) as i32 as f64);
        }
    } else if expression.contains("==") {
        let parts: Vec<&str> = expression.split("==").collect();
        if parts.len() == 2 {
            let left = evaluate_arithmetic(parts[0])?;
            let right = evaluate_arithmetic(parts[1])?;
            return Ok((left == right) as i32 as f64);
        }
    } else if expression.contains(">=") {
        let parts: Vec<&str> = expression.split(">=").collect();
        if parts.len() == 2 {
            let left = evaluate_arithmetic(parts[0])?;
            let right = evaluate_arithmetic(parts[1])?;
            return Ok((left >= right) as i32 as f64);
        }
    } else if expression.contains("<=") {
        let parts: Vec<&str> = expression.split("<=").collect();
        if parts.len() == 2 {
            let left = evaluate_arithmetic(parts[0])?;
            let right = evaluate_arithmetic(parts[1])?;
            return Ok((left <= right) as i32 as f64);
        }
    } else {
        // 如果没有比较运算符，则尝试作为算术运算处理
        return evaluate_arithmetic(&expression);
    }

    Err("Invalid expression".to_string())
}

// 提取括号内的表达式
fn extract_inner_expression(expression: &str) -> Result<String, String> {
    let mut open_paren = 0;
    let mut close_paren = 0;
    let mut inner_expr = String::new();
    let mut inside_paren = false;

    let mut iter = expression.chars().enumerate();
    let mut start = None;

    // 遍历字符，处理括号
    while let Some((i, c)) = iter.next() {
        if c == '(' {
            if open_paren == 0 {
                start = Some(i); // 记录括号的开始位置
            }
            open_paren += 1;
        } else if c == ')' {
            close_paren += 1;
            if open_paren == close_paren {
                if let Some(start_pos) = start {
                    inner_expr = expression[start_pos + 1..i].to_string(); // 获取括号内的表达式
                }
                break;
            }
        }
    }

    if inner_expr.is_empty() {
        Err("No valid expression inside parentheses".to_string())
    } else {
        Ok(inner_expr)
    }
}

// 处理算术运算表达式
fn evaluate_arithmetic(expression: &str) -> Result<f64, String> {
    let sanitized_expr = expression.replace(" ", ""); // 去除空格
    meval::eval_str(&sanitized_expr).map_err(|e| format!("Failed to evaluate: {}", e))
}

#[derive(Debug)]
pub struct SplitExpressionToken{
    expression: String,
    symbol: Option<char>,
    next : Option<Vec<SplitExpressionToken>>,
}

impl SplitExpressionToken {
    fn new(expression: String) -> Self {
        SplitExpressionToken{
            expression,
            symbol:None,
            next:None
        }
    }
}

fn split_expression_str(expression: &str) -> Vec<SplitExpressionToken> {
    split_expression(crate::SplitExpressionToken::new(expression.to_string()),None)
}

fn split_expression(pre:SplitExpressionToken,next_expression: Option<&str>) -> Vec<SplitExpressionToken> {
    let expression = pre.expression;
    // if next_expression.is_some() && expression==next_expression.unwrap() {
    //     return Vec::new();
    // }
    let mut result:Vec<SplitExpressionToken> = Vec::new();
    let mut balance = 0;  // 用来跟踪括号的平衡
    let mut start = 0;    // 分割开始的位置

    // 遍历字符串
    let mut i = 0;
    while i < expression.len() {
        let c = expression[i..].chars().next().unwrap();
        println!("----: {}", c);
        match c {
            '(' => balance += 1, // 增加括号平衡
            ')' => balance -= 1, // 减少括号平衡
            _ => {
                // 只有在括号外部的逻辑运算符才是分割点
                if balance == 0 && (c == '&' || c == '|') {
                    // 检查下一个字符，以确定是否是 "&&" 或 "||"
                    if i + 1 < expression.len() {
                        let next_char = expression[i + 1..].chars().next().unwrap();
                        if (c == '&' && next_char == '&') || (c == '|' && next_char == '|') {

                            let expression = expression[start..i].to_string();

                            // 找到 "&&" 或 "||"，这是一个有效的分割点
                            let mut token = SplitExpressionToken{
                                expression:expression.clone(),
                                symbol: Some(c),
                                next : None,
                            };

                            // let next_list = split_expression(token, Some(&*expression.as_str()));

                            // token.next = Some(next_list);

                            result.push(token);
                            start = i + 2; // 跳过运算符
                            i += 1; // 跳过下一个字符
                        }
                    }
                }
            }
        }
        i += 1;
    }

    // 添加最后一部分
    if start < expression.len() {
        let expression = expression[start..i].to_string();
        // let next_list = split_expression(expression.as_str());
        let token = SplitExpressionToken{
            expression:expression,
            symbol: None,
            next: None,
        };
        result.push(token);
    }

    result
}


fn main() {
    //
    // let expression = "(10 > 5 && 3 < 5) || (10 > 5 || 3 < 5) && (10 > 5 || 3 < 5) && 10>6 && ((1>2 && 2>3) && 10>6)";
    // // let expression = "10<1 && 12<13";
    // let parts = split_expression_str(expression);
    //
    //
    // for part in parts {
    //     println!("{:?}", part);
    // }
    //
    // let expressions = vec![
    //     "(10 > 5 && 3 < 5) || (10 > 5 || 3 < 5) && (10 > 5 || 3 < 5)", // 复杂表达式
    //     "18 > 15",
    //     "10 + 5 * 2",
    //     "20 == 20",
    //     "50 / 2 + 3 < 30",
    //     "true && false",
    //     "true || false",
    //     "10 > 5 && 3 < 5", // 使用 && 运算符
    //     "10 > 5 || 3 < 5",  // 使用 || 运算符
    // ];
    //
    // for expr in expressions {
    //     match evaluate_expression(expr) {
    //         Ok(result) => println!("Expression: {} => Result: {}", expr, result),
    //         Err(e) => println!("Expression: {} => Error: {}", expr, e),
    //     }
    // }
    let vec = vec!["a", "b", "c", "d", "e", "f"];

    println!("{:?}", vec[0..=2].to_vec()); // 转换为 Vec<&str>
    println!("{:?}", vec[0..3].to_vec()); // 转换为 Vec<&str>

    let template =
r#"#if($lombokEnable)
#if($lombokEnable)import lombok.*;#end#if($lombokEnable)import lombok.*;
#if (${swagger2Enable})
import io.swagger.annotations.ApiModel;
import io.swagger.annotations.ApiModelProperty;
#end
#end
#end
"#;

   let result =  tag_parse::calculate_tag_positions(template);
    println!("{:#?}", result);
    let final_positions = tag_parse::calculate_tag_final_positions(result);
    println!("{}",  serde_json::to_string(&final_positions).unwrap());

    let rr = tag_parse::test(template, final_positions);
    println!("{:#?}", rr);

}




