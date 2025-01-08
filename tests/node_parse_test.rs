
use velocity::read_file;
use velocity::node::node_parse;

#[test]
pub fn build_tree_from_template_test() {

    let template_path = "tests/if/if_1.vm";
    let template = if let Ok(content) = read_file(template_path) {
        content // 直接将 String 赋值给 template
    } else {
        String::new() // 返回一个空字符串作为默认值
    };

    println!("template: {:?}",template);



    let tree = node_parse::build_tree_from_template(&template);

    println!("tree: {:?}",serde_json::to_string(&tree.unwrap()));

    println!("template: {:#?}",&template[34..74]);
    // println!("template: {:?}",&template[147..158]);
}



#[test]
pub fn parse_template_test(){
    let template_path = "tests/if/if_1.vm";
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