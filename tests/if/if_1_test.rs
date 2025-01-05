use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use velocity;
use velocity::{read_file, VelocityEngine};
use velocity::tag::tag_parse;

#[test]
fn tag_position_test() {

    let template_path = "tests/if/if_1.vm";
    let template = if let Ok(content) = read_file(template_path) {
        content // 直接将 String 赋值给 template
    } else {
        String::new() // 返回一个空字符串作为默认值
    };

    println!("template: {}",template);

    let result =  tag_parse::calculate_tag_positions(&template);

    println!("result: {:?}",result);

}