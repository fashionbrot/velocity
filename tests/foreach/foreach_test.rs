use serde::{Deserialize, Serialize};
use velocity;
use velocity::{read_file, render_from_object};
use velocity::token::token_parse::get_tokens;
use crate::log_config;

#[derive(Debug,Serialize,Deserialize)]
struct User{
    pub age: i32,
    pub list:Vec<i32>,
}

#[test]
pub fn foreach_tokens_test(){
    log_config::print_debug_log();

    let template_path = "tests/foreach/foreach.vm";
    let template = if let Ok(content) = read_file(template_path) {
        content // 直接将 String 赋值给 template
    } else {
        String::new() // 返回一个空字符串作为默认值
    };

    let user = User{
        age: 18,
        list: vec![1,2,3],
    };

    let result = get_tokens(&template);
    println!("{:#?}", result);
}

#[test]
pub fn foreach_test() {

    log_config::print_debug_log();

    let template_path = "tests/foreach/foreach.vm";
    let template = if let Ok(content) = read_file(template_path) {
        content // 直接将 String 赋值给 template
    } else {
        String::new() // 返回一个空字符串作为默认值
    };

    let user = User{
        age: 18,
        list: vec![1,2,3],
    };


    println!("template: {:?}", template);
    for x in 0..2  {
        let output_result = render_from_object(&template,&user);
        if let Ok(output) = output_result{
            println!("------------------------------------\n{}", output);
            println!("----------------------------------------------------------------------")
        }
    }
}