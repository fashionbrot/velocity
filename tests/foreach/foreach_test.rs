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

#[derive(Debug,Serialize,Deserialize)]
struct Project{
    name:String,
    user_list:Vec<ProjectUser>,
}
#[derive(Debug,Serialize,Deserialize)]
struct ProjectUser{
    name: String,
}
#[derive(Debug,Serialize,Deserialize)]
struct Template{
    project_list:Vec<Project>,
}
#[test]
fn foreach_array_test(){
    log_config::print_debug_log();

    let template_path = "tests/foreach/foreach_array.vm";
    let template = if let Ok(content) = read_file(template_path) {
        content // 直接将 String 赋值给 template
    } else {
        String::new() // 返回一个空字符串作为默认值
    };

    let user1 = ProjectUser{
        name: "张三".to_string(),
    };
    let user2 = ProjectUser{
        name: "李四".to_string(),
    };

    let user3 = ProjectUser{
        name: "王五".to_string(),
    };
    let user4 = ProjectUser{
        name: "小李子".to_string(),
    };


    let p1 = Project{
        name: "项目1".to_string(),
        user_list: vec![user1,user2],
    };

    let p2 = Project{
        name: "项目2".to_string(),
        user_list: vec![user3,user4],
    };

    let entity = Template{
        project_list: vec![p1,p2],
    };


    let output = render_from_object(&template,&entity);
    if let Ok(output) = output {
        println!("------------------------------------------------------------------\n{}", output);
        println!("------------------------------------------------------------------");
    }
}