use std::io::stdout;
use std::iter::Enumerate;
use regex::{Match, Matches};
use velocity::node::node::{Node, NodePosition, PairPosition, TreeNode};
use velocity::read_file;
use velocity::tag::tag_parse::{get_else_list, TagFinalPosition, TagPosition, TAGS, TAGS_PATTERN};

#[test]
pub fn test() {

    let template_path = "tests/if/if_1.vm";
    let template = if let Ok(content) = read_file(template_path) {
        content // 直接将 String 赋值给 template
    } else {
        String::new() // 返回一个空字符串作为默认值
    };

    println!("template: {:?}",template);


    let tree = build_tree_from_template(&template);

    println!("tree: {:?}",tree);
}


pub fn build_tree_from_template(template:&str) ->Result<Vec<TreeNode>, String> {
    if template.is_empty() {
        return Err("template empty".to_string()); ;
    }

    //生成开始结束标签
    let mut pair_position_list:Vec<PairPosition> = Vec::new();
    let mut stack:Vec<NodePosition> = Vec::new(); // 用来存储开始标签的索引
    for (index,capture) in TAGS_PATTERN.find_iter(template).enumerate() {
        let node_position = NodePosition::build(&capture);
        node_position.print();
        let tag_name = &node_position.tag_name;

        if tag_name == "#end" {
            if let Some(position) = stack.pop() {
                pair_position_list.push(PairPosition::new( position,node_position.clone()));
            } else {
                // 如果没有找到匹配的开始标签，说明不匹配，报错
                return Err(format!("Unmatched #end at index {}", tag_name));
            }
        }else if tag_name == "#else" {
            if let Some(position) = stack.pop() {
                pair_position_list.push(PairPosition::new( position,node_position.clone()));
                stack.push(node_position);
            }
        }else if tag_name == "#elseif" {
            if let Some(position) = stack.pop() {
                pair_position_list.push(PairPosition::new( position,node_position.clone()));
                stack.push(node_position);
            }
        }else {
            stack.push(node_position);
        }
    }
    println!("pair position list: {:#?}",pair_position_list);


    //生成树形结构
    pair_position_list.sort_by_key(|tag| std::cmp::Reverse(tag.begin));
    println!("111111111111111111pair position list: {:#?}",pair_position_list);
    let mut position_tree:Vec<PairPosition> = Vec::new();
    let mut position_temp:Vec<PairPosition>  = Vec::new();
    for pair in &pair_position_list {
        let mut p = pair.clone();
        let begin = p.begin;
        let finish = p.finish;

        println!("begin: {:?} finish: {:?}", begin, finish);

        //获取子节点&& 排序
        // let child = get_child(&pair_position_list,&p);
        let child:Vec<PairPosition> = position_temp
            .iter()
            .filter(|t|  begin< t.begin && finish> t.finish)
            .cloned()
            .collect();
        println!("child: {:?}",child);
        p.child = Some(child.clone());
        // //删除子节点在 position_temp 中的数据
        if !child.is_empty() {
            child.iter().for_each(|child| {
                if let Some(index) = position_temp.iter().position(|x| x.begin == child.begin && x.finish == child.finish) {
                    position_temp.swap_remove(index);
                }
            });

        }




        //解析else elseif
        let mut else_node_list = vec![];
        let mut current_end = finish;
        while let Some(branch) = get_branch(&position_temp, current_end) {
            current_end =branch.finish;
            else_node_list.push(branch.clone());
            if let Some(index) = position_temp.iter().position(|x| x.begin == branch.begin && x.finish == branch.finish) {
                position_temp.swap_remove(index);
            }
        }
        p.branch = Some(else_node_list);

        position_temp.push(p.clone());

        if p.is_root(&pair_position_list){
            println!("-------------------------------{:?}",p);
            position_tree.push(p.clone());
        }
    }

    println!("position tree: {:#?}",position_tree);


    let mut tree: Vec<TreeNode> = Vec::new();


    Ok(tree)
}

pub fn get_child(pair_list: &[PairPosition], current: &PairPosition) -> Vec<PairPosition> {
    // 遍历 tags 查找与当前标签匹配的子标签
    pair_list.iter()
        .filter(|tag| tag.is_child(current))
        .cloned()
        .collect()
}




pub fn get_branch(tags: &[PairPosition], current_end: u32) -> Option<&PairPosition> {
    tags.iter()
        .find(|tag| tag.begin == current_end)
}