use velocity::{read_file, render_default, render_from_object};
use crate::log_config;

#[test]
pub fn set_test(){
    log_config::print_debug_log();

    let template_path = "tests/set/set.vm";
    let template = if let Ok(content) = read_file(template_path) {
        content // 直接将 String 赋值给 template
    } else {
        String::new() // 返回一个空字符串作为默认值
    };


    println!("template: {:?}", template);
    for x in 0..1  {
        let output_result = render_default(&template);
        if let Ok(output) = output_result{
            println!("------------------------------------\n{}", output);
            println!("----------------------------------------------------------------------")
        }
    }
}