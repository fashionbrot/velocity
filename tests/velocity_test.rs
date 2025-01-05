mod r#if;

use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use velocity;
use velocity::{read_file, VelocityEngine};


#[derive(Clone, Debug, PartialEq,Serialize,Deserialize)]
struct User{
    pub username: String,
}

#[test]
pub fn test(){
    let template_path = "tests/entity.vm";
    let template = if let Ok(content) = read_file(template_path) {
        content // 直接将 String 赋值给 template
    } else {
        String::new() // 返回一个空字符串作为默认值
    };

    println!("template: {}",template);

    let user1 = User {
        username: "张三".to_string(),
    };
    let user2 = User {
        username: "李四".to_string(),
    };
    let user_list = vec![user1,user2];

    // 将 Vec<User> 转换为 serde_json::Value::Array
    let user_list_json = serde_json::to_value(user_list).expect("Failed to serialize users");

    let mut engine = VelocityEngine::new();
    engine.insert("age",Value::Number(Number::from(19)));
    engine.insert("userList",user_list_json);
    engine.insert("one",Value::Number(Number::from(1)));
    engine.insert("rust",Value::String("rust 2025".to_string()));


    let output = engine.render(template.as_str());

    print!("{}",output);
}