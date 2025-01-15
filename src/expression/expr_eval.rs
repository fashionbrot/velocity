
use evalexpr::{ Value};

pub fn eval(input: &str) -> bool {
    // 计算表达式
    let result = evalexpr::eval(input);

    // 处理计算错误
    if result.is_err() {
        // println!("Error evaluating expression: {}", result.err().unwrap());
        return false; // 返回一个默认值，表示错误时的结果
    }

    // 获取计算结果
    let value = result.unwrap();

    // 将结果转换为布尔值，如果转换失败，则返回 false
    value.as_boolean().unwrap_or_else(|_| {
        // println!("The result is not a boolean value.");
        false
    })
}

pub fn eval_value(input: &str) -> Result<Value, String> {
    let result = evalexpr::eval(input);
    // 处理计算错误
    if result.is_err() {
        return Err(format!("Error evaluating expression: {}", result.err().unwrap())); // 返回一个默认值，表示错误时的结果
    }
    // 获取计算结果
    let value = result.unwrap();
    Ok(value.clone())
}




pub fn is_valid_expression(input: &str) -> bool {
    // 尝试评估表达式
    let result = evalexpr::eval(input);

    // 如果评估成功，返回 true；如果失败，返回 false
    result.is_ok()
}

pub fn eval_expression(input: &str) -> Result<bool, String> {
    // 判断表达式是否有效
    if !is_valid_expression(input) {
        return Err("Invalid expression".to_string());
    }

    // 评估表达式
    let result = evalexpr::eval(input);

    // 处理计算结果
    match result {
        Ok(value) => {
            // 尝试将结果转换为布尔值
            match value.as_boolean() {
                Ok(b) => Ok(b),
                Err(e) => Err("Result is not a boolean".to_string()),
            }
        }
        Err(e) => Err(format!("Error evaluating expression: {}", e)),
    }
}


#[cfg(test)]
mod tests {
    use crate::expression::expr_eval::eval_expression;

    #[test]
    fn main() {
        let input = "true && false";
        match eval_expression(input) {
            Ok(result) => println!("Result: {}", result),
            Err(e) => println!("Error: {}", e),
        }

        let invalid_input = "true &&";
        match eval_expression(invalid_input) {
            Ok(result) => println!("Result: {}", result),
            Err(e) => println!("Error: {}", e),
        }
    }

}
