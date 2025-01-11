use serde_json::{json, Number, Value};
use velocity::{read_file, VelocityEngine};
use velocity::token::token_parse;
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

    let template_path = "tests/if/if2.vm";
    let template = if let Ok(content) = read_file(template_path) {
        content // 直接将 String 赋值给 template
    } else {
        String::new() // 返回一个空字符串作为默认值
    };

    println!("template: {:?}", template);
    // log::debug!("14-34 {:?}",&template[49..307]);

    let result = token_parse::parse_position(&template, 0);
    log::debug!("result:{:#?}",result);
    println!("result: {:#?}", result);
    if let Ok(mut list) = result {
        // for x in list {
        //     log::debug!("xxxxx{:?}", &template[x.first_start..x.last_end]);
        // }
        println!("position_list {:#?}", list);
        let tokens = token_parse::position_to_tokenizer(&template, &mut list);
        println!("tokens: {:#?}", tokens);

        if let Some(tokens) = tokens {

            // let user1 = User {
            //     username: "张三".to_string(),
            // };
            // let user2 = User {
            //     username: "李四".to_string(),
            // };
            // let user_list = vec![user1,user2];

            let list = Value::Array(vec![
                json!({"name": "item1", "value": 1}),
                json!({"name": "item2", "value": 2}),
                json!({"name": "item3", "value": 3}),
            ]);

            let mut engine = VelocityEngine::new();
            engine.insert("age", Value::Number(Number::from(19)));
            engine.insert("one", Value::Number(Number::from(1)));
            engine.insert("rust", Value::String("rust 2025".to_string()));
            engine.insert("list",list);

            let mut context = engine.context;

            let value = token_parse::parse_token(&tokens, &mut context);
            if let Some(value) = value {
                println!("------------------------------------\n{}", value);
                println!("----------------------------------------------------------------------")
            }
        }
    }
}

#[test]
pub fn parse(){
    log_config::print_debug_log();

    let template_path = "tests/if/if2.vm";
    let template = if let Ok(content) = read_file(template_path) {
        content // 直接将 String 赋值给 template
    } else {
        String::new() // 返回一个空字符串作为默认值
    };

    println!("template: {:?}", template);

    for x in 0..100  {
        if let Some (tokens) = token_parse::get_tokens(&template){
            let list = Value::Array(vec![
                json!({"name": "item1", "value": 1}),
                json!({"name": "item2", "value": 2}),
                json!({"name": "item3", "value": 3}),
            ]);

            let mut engine = VelocityEngine::new();
            engine.insert("age", Value::Number(Number::from(19)));
            engine.insert("one", Value::Number(Number::from(1)));
            engine.insert("rust", Value::String("rust 2025".to_string()));
            engine.insert("list",list);

            let mut context = engine.context;

            let value = token_parse::parse_token(&tokens, &mut context);
            if let Some(value) = value {
                println!("------------------------------------\n{}", value);
                println!("----------------------------------------------------------------------")
            }
        }
    }




}


