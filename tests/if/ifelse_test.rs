use velocity::node::node_parse;
use velocity::read_file;
use velocity::tag::tag_parse;

#[test]
fn if_else_test () {

    let template_path = "tests/if/ifelse1.vm";
    let template = if let Ok(content) = read_file(template_path) {
        content // 直接将 String 赋值给 template
    } else {
        String::new() // 返回一个空字符串作为默认值
    };

    println!("template: {:?}",template);

    let tree = node_parse::build_tree_from_template(&template);

    println!("tree: {:?}",tree);

    if let Ok(tree) = tree {
        let expression_list = node_parse::parse_template(0,&template,&tree);
        if let Some(expression_list) = expression_list {
            println!("expression_list: {:#?}",expression_list);
        }
    }
}