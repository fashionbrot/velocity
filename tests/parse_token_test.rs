use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::{json, Number, Value};
use velocity::{parse_template, parse_template_object, read_file};
use velocity::token::token_parse;
use velocity::token::token_parse::Tokenizer;
use crate::log_config;


#[test]
fn parse_position_test() {
    log_config::print_debug_log();

    let template_path = "tests/if/if_1.vm";
    let template = if let Ok(content) = read_file(template_path) {
        content // 直接将 String 赋值给 template
    } else {
        String::new() // 返回一个空字符串作为默认值
    };

    println!("template: {:?}", template);
    // println!("template: {:?}", &template[487..487+36]);

    // // for x in template.chars().enumerate(){
    // //     println!("{} {}",x.1,x.0 );
    // // }
    //
    let result = token_parse::parse_position(&template, 0);

    println!("result: {:#?}", result);

    if let Ok(list) = result {
        for x in list {
            println!("{:?}", &template[x.first_start..x.last_end]);
        }
    }
}


#[test]
fn parse_position_token_test() {
    log_config::print_debug_log();
    let template_path = "tests/if/if_1.vm";
    let template = if let Ok(content) = read_file(template_path) {
        content // 直接将 String 赋值给 template
    } else {
        String::new() // 返回一个空字符串作为默认值
    };

    println!("template: {:?}", template);
    // log::debug!("14-34 {:?}",&template[49..307]);

    let result = token_parse::parse_position(&template, 0);
    println!("result:{:#?}",result);


}

#[test]
pub fn prase_template_token(){
    log_config::print_debug_log();

    let template_path = "tests/if/if_1.vm";
    let template = if let Ok(content) = read_file(template_path) {
        content // 直接将 String 赋值给 template
    } else {
        String::new() // 返回一个空字符串作为默认值
    };

    let tokens = token_parse::get_tokens(&template);
    println!("tokens: {:#?}", tokens);
}

#[test]
pub fn parse(){
    log_config::print_debug_log();

    let template_path = "tests/if/if_1.vm";
    let template = if let Ok(content) = read_file(template_path) {
        content // 直接将 String 赋值给 template
    } else {
        String::new() // 返回一个空字符串作为默认值
    };

    println!("template: {:?}", template);
    let list = Value::Array(vec![
        json!({"name": "item1", "value": 1}),
        json!({"name": "item2", "value": 2}),
        json!({"name": "item3", "value": 3}),
    ]);

    let mut context:HashMap<String,Value> = HashMap::new();
    context.insert("age".to_string(), Value::Number(Number::from(19)));
    context.insert("one".to_string(), Value::Number(Number::from(1)));
    context.insert("rust".to_string(), Value::String("rust 2025".to_string()));
    context.insert("list".to_string(),list);

    if let Ok(output) = parse_template(&template,&mut context){
        println!("------------------------------------\n{}", output);
        println!("----------------------------------------------------------------------")
    }

}

#[derive(Debug,Serialize,Deserialize)]
struct User{
    pub age: i32,
    pub name: String,
    pub list: Vec<String>,
}

#[test]
pub fn parse_(){


    log_config::print_debug_log();

    let template_path = "tests/if/if_1.vm";
    let template = if let Ok(content) = read_file(template_path) {
        content // 直接将 String 赋值给 template
    } else {
        String::new() // 返回一个空字符串作为默认值
    };

    let user = User{
        age: 18,
        name: "张三".to_string(),
        list: vec![String::from("张三"), String::from("李四"), String::from("王五")],
    };


    println!("template: {:?}", template);
    for x in 0..1  {
        let output_result = parse_template_object(&template,&user);
        if let Ok(output) = output_result{
            println!("------------------------------------\n{}", output);
            println!("----------------------------------------------------------------------")
        }
    }

}


