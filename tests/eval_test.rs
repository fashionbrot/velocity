use std::collections::HashMap;
use meval::{eval_str_with_context, Context};
use regex::Regex;

fn contains_logical_operators(input: &str) -> bool {
    input.contains("&&") || input.contains("||") || input.contains("!") || input.contains("==") || input.contains("!=")
}

fn evaluate_expression(input: &str, context: &Context) -> Result<f64, String> {
    if contains_logical_operators(input) {
        match eval_str_with_context(input, context) {
            Ok(result) => Ok(result),
            Err(e) => Err(format!("Error evaluating expression: {}", e)),
        }
    } else {
        Err("No logical operators found".to_string())
    }
}

#[test]
fn main() {
    // 定义上下文，包含变量的值
    let mut context: Context = Context::new();
    context.var("x", 5.0);
    context.var("y", 3.0);
    context.var("a", 1.0);
    context.var("b", 1.0);
    context.var("c", 2.0);
    context.var("d", 3.0);

    let expr1 = "x > 5 && y < 10";
    let expr2 = "a == b || c != d";
    let expr3 = "x + y";

    let r = meval::eval_str("1+2*3/5").unwrap();

    println!("1 + 2 = {}", r);
}


fn contains_meval(input: &str) -> bool {
    // 正则表达式匹配数学运算符、逻辑运算符、函数和常量
    let re = Regex::new(r"(\+|\-|\*|\/|\%|\^|&&|\|\||==|!=|sqrt|abs|exp|ln|sin|cos|tan|asin|acos|atan|atan2|sinh|cosh|tanh|asinh|acosh|atanh|floor|ceil|round|signum|max|min|pi|e)").unwrap();

    // 检查输入字符串是否包含匹配的内容
    re.is_match(input)
}

#[test]
fn main2() {
    let expr1 = "x > 5 && y < 10";
    let expr2 = "sqrt(x) + y";
    let expr3 = "a == b || c != d";
    let expr4 = "x + y";

    println!("Does '{}' contain meval operators or functions? {}", expr1, contains_meval(expr1));
    println!("Does '{}' contain meval operators or functions? {}", expr2, contains_meval(expr2));
    println!("Does '{}' contain meval operators or functions? {}", expr3, contains_meval(expr3));
    println!("Does '{}' contain meval operators or functions? {}", expr4, contains_meval(expr4));
}
