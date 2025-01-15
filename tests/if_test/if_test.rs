use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use velocity_template;
use velocity_template::{read_file, render};


#[derive(Serialize, Deserialize, Debug)]
struct User {
    username: String
}

#[test]
pub fn test(){
    let template_path = "tests/if_test/if.vm";
    let template = if let Ok(content) = read_file(template_path) {
        content // 直接将 String 赋值给 template
    } else {
        String::new() // 返回一个空字符串作为默认值
    };

    println!("template: {:?}",template);

    let user1 = User {
        username: "张三".to_string(),
    };
    let user2 = User {
        username: "李四".to_string(),
    };
    let user_list = vec![user1,user2];

    // 将 Vec<User> 转换为 serde_json::Value::Array
    let user_list_json = serde_json::to_value(user_list).expect("Failed to serialize users");

    let mut engine = HashMap::<String, Value>::new();
    engine.insert(String::from("age"), Value::Number(Number::from(17)));
    engine.insert(String::from("userList"), user_list_json);
    engine.insert(String::from("one"), Value::Number(Number::from(1)));
    engine.insert(String::from("rust"), Value::String("rust 2025".to_string()));


    let output = render(&template,&mut engine);
    if let Ok(output) = output {
        println!("output:\n{:#?}",output);
    }

}

