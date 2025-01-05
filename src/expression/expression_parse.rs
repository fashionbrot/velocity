use std::time::Instant;
use tokio::io::split;
use regex::Regex;

// #[derive(Clone, Debug, PartialEq)]
// pub struct Expression{
//     expression: String,
//     symbol: Option<char>,
//     next : Option<Vec<crate::expression::expression_parse::SplitExpressionToken>>,
// }


pub fn split_expression(expression: &str){

}
pub fn execute_expression(expression: &str) -> Result<bool, String> {
    if expression.trim() =="true" {
        return Ok(true);
    }
    if expression.trim() == "false" {
        return Ok(false);
    }
    let re = Regex::new(r"(?P<left>[-\d\.]+)\s*(?P<operator>>|<|==|>=|<=)\s*(?P<right>[-\d\.]+)").unwrap();

    if let Some(caps) = re.captures(expression) {
        let left_str = &caps["left"];
        let right_str = &caps["right"];
        let operator = &caps["operator"];


        // 调用 evaluate_arithmetic 来解析左右操作数
        let l = evaluate_arithmetic(left_str);
        let r = evaluate_arithmetic(right_str);
        if l.is_ok() && r.is_ok() {
            let left = l.unwrap();
            let right = r.unwrap();
            // 根据 operator 执行对应的比较
            match operator {
                ">" => Ok(left > right),
                "<" => Ok(left < right),
                "==" => Ok(left == right),
                ">=" => Ok(left >= right),
                "<=" => Ok(left <= right),
                _ => Err("Invalid operator".to_string()), // 如果出现无效的操作符
            }
        }else {
            Err("Invalid expression".to_string())
        }


    } else {
        Err("if 条件没有包含 >、<、==、>=、<=".to_string()) // 如果没有匹配到比较运算符
    }
}

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



    // else {
    //     // 如果没有比较运算符，则尝试作为算术运算处理
    //     return evaluate_arithmetic(&expression);
    // }

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


#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test(){
        // let expression = "(10 > 5 && 3 < 5) || (10 > 5 || 3 < 5) && (10 > 5 || 3 < 5) && 10>6 && ((1>2 && 2>3) && 10>6)";
        // // let expression = "10<1 && 12<13";
        // let parts = split_expression_str(expression);
        //
        //
        // for part in parts {
        //     println!("{:?}", part);
        // }

        let expressions = vec![
            "true",
            "false",
            "1>10",
            "1<10",
            "(10 > 5 && 3 < 5) || (10 > 5 || 3 < 5) && (10 > 5 || 3 < 5)", // 复杂表达式
            "18 > 15",
            "10 + 5 * 2",
            "20 == 20",
            "50 / 2 + 3 < 30",
            "true && false",
            "true || false",
            "10 > 5 && 3 < 5", // 使用 && 运算符
            "10 > 5 || 3 < 5",  // 使用 || 运算符
        ];

        for expr in expressions {
            match evaluate_expression(expr) {
                Ok(result) => println!("Expression: {} => Result: {}", expr, result),
                Err(e) => println!("Expression: {} => Error: {}", expr, e),
            }
        }
    }
}




