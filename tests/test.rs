use velocity::node::node;
use velocity::read_file;

#[test]
pub fn test() {

    let template_path = "tests/if/if_1.vm";
    let template = if let Ok(content) = read_file(template_path) {
        content // 直接将 String 赋值给 template
    } else {
        String::new() // 返回一个空字符串作为默认值
    };

    println!("template: {:?}",template);


    let tree = node::build_tree_from_template(&template);

    println!("tree: {:?}",tree);
}

