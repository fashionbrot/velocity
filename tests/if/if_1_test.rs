use serde_json::{Number, Value};
use velocity;
use velocity::{read_file, VelocityEngine};
use crate::User;



#[test]
pub fn test(){
    let template_path = "tests/if/if_1.vm";
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

    let mut engine = VelocityEngine::new();
    engine.insert("age",Value::Number(Number::from(17)));
    engine.insert("userList",user_list_json);
    engine.insert("one",Value::Number(Number::from(1)));
    engine.insert("rust",Value::String("rust 2025".to_string()));


    let output = engine.render(template.as_str());

    println!("output:{:?}",output);
    assert_eq!(output,"    else 18\n")
}

#[test]
fn ttt(){
    let test_str2 = "    \r\n    18岁了，55\r\n    ";
    println!("test_str2: {:?}",remove_surrounding_crlf(test_str2));
}

fn remove_surrounding_crlf(input: &str) -> String {

    let start_ = input.find("\r\n");
    let end_ = input.rfind("\r\n");

    // 如果没有找到 \r\n，则直接返回原始字符串
    if start_.is_none() || end_.is_none() {
        return input.to_string();
    }

    let start = start_.unwrap();
    let end = end_.unwrap();

    let start_text = &input[0..start];
    let m_text = &input[(start+2)..end];
    let end_text = &input[end+2..];

    format!("{}{}{}", start_text, m_text, end_text)
}
