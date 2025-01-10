use velocity::read_file;
use velocity::token::token_parse;
use crate::log_config;

#[test]
fn parse_token_test() {

    log_config::print_debug_log();

    let template_path = "tests/if/if_1.vm";
    let template = if let Ok(content) = read_file(template_path) {
        content // 直接将 String 赋值给 template
    } else {
        String::new() // 返回一个空字符串作为默认值
    };

    println!("template: {:?}",template);


    let result = token_parse::parse_token(&template);
    println!("result: {:?}",result);

    println!("result: {:?}",&template[11..39]);
    println!("result: {:?}",&template[57..72]);
    println!("result: {:?}",&template[34..78]);

}