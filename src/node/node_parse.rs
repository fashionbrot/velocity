use crate::node::{text_node, ExpressionNode, NodePosition, PositionTree, TAGS_PATTERN};
use crate::tag::tag_parse;

pub fn build_tree_from_template(template:&str) ->Result<Vec<PositionTree>, String> {
    if template.is_empty() {
        return Err("template empty".to_string()); ;
    }

    //生成开始结束标签
    let mut pair_position_list:Vec<PositionTree> = Vec::new();
    let mut stack:Vec<NodePosition> = Vec::new(); // 用来存储开始标签的索引
    for (index,capture) in TAGS_PATTERN.find_iter(template).enumerate() {
        let node_position = NodePosition::build(&capture);
        node_position.print();
        let tag_name = &node_position.tag_name;

        if tag_name == "#end" {
            if let Some(position) = stack.pop() {
                pair_position_list.push(PositionTree::new(position, node_position.clone()));
            } else {
                // 如果没有找到匹配的开始标签，说明不匹配，报错
                return Err(format!("Unmatched #end at index {}", tag_name));
            }
        }else if tag_name == "#else" {
            if let Some(position) = stack.pop() {
                pair_position_list.push(PositionTree::new(position, node_position.clone()));
                stack.push(node_position);
            }
        }else if tag_name == "#elseif" {
            if let Some(position) = stack.pop() {
                pair_position_list.push(PositionTree::new(position, node_position.clone()));
                stack.push(node_position);
            }
        }else {
            stack.push(node_position);
        }
    }
    println!("pair position list: {:#?}",pair_position_list);

    if pair_position_list.is_empty(){
        return Ok(Vec::new());
    }

    //生成树形结构
    pair_position_list.sort_by_key(|tag| std::cmp::Reverse(tag.start));
    println!("111111111111111111  pair position list: {:#?}",pair_position_list);
    let mut position_tree:Vec<PositionTree> = Vec::new();
    let mut position_temp:Vec<PositionTree>  = Vec::new();
    for pair in &pair_position_list {
        let mut p = pair.clone();
        let start = p.start;
        let end = p.end;

        println!("start: {:?} finish: {:?}", start, end);

        //获取子节点&& 排序
        // let child = get_child(&pair_position_list,&p);
        let child:Vec<PositionTree> = position_temp
            .iter()
            .filter(|t|  start< t.start && end> t.end)
            .cloned()
            .collect();
        println!("child: {:?}",child);
        // //删除子节点在 position_temp 中的数据
        if !child.is_empty() {
            child.iter().for_each(|child| {
                if let Some(index) = position_temp.iter().position(|x| x.start == child.start && x.end == child.end) {
                    position_temp.swap_remove(index);
                }
            });
            let mut children = child.clone();
            children.sort_by_key(|tag| tag.start);
            p.child = Some(children);
            p.child_last_end = Some(get_child_last_end(&p));
        }


        //解析else elseif
        let mut branch_list = vec![];
        let mut current_end = end;
        while let Some(branch) = get_branch(&position_temp, current_end) {
            current_end =branch.end;
            branch_list.push(branch.clone());
            if let Some(index) = position_temp.iter().position(|x| x.start == branch.start && x.end == branch.end) {
                position_temp.swap_remove(index);
            }
        }
        if !branch_list.is_empty() {
            p.branch = Some(branch_list);
        }



        position_temp.push(p.clone());

        if p.is_root(&pair_position_list){
            println!("root -------------------------------{:?}",p);
            position_tree.push(p.clone());
        }
    }

    println!("position tree: {:#?}",position_tree);

    position_tree.sort_by_key(|tag| tag.start);
    Ok(position_tree)
}

pub fn get_branch(tags: &[PositionTree], current_end: usize) -> Option<&PositionTree> {
    tags.iter()
        .find(|tag| tag.start == current_end)
}


// fn get_child_last_end(tree: &PositionTree) -> usize {
//     let mut max_end = 0; // 初始化为当前节点的 `end`
//
//     // 检查子节点集合
//     if let Some(children) = &tree.child {
//         let last = children.iter().last();
//         if let Some(last) = last {
//             max_end = last.end;
//             let child_last_end = get_child_last_end(last);
//             if child_last_end ==0 {
//                 return max_end;
//             }
//         }
//     }
//
//
//     max_end
// }

fn get_child_last_end(tree: &PositionTree) -> usize {
    // 初始化为当前节点的 end

    let mut child_max = 0 ;
    // 检查子节点集合
    if let Some(children) = &tree.child {
        for child in children {
            child_max  = child.end;
            let child_end = get_child_last_end(child);
            child_max = child_max.max(child_end); // 更新最大值
        }
    }

    let mut branch_max = 0 ;
    // 检查分支集合
    if let Some(branches) = &tree.branch {
        for branch in branches {
            branch_max = branch.end;
            let branch_end = get_child_last_end(branch);
            branch_max = branch_max.max(branch_end); // 更新最大值
        }
    }

    child_max.max(branch_max)
}





pub fn parse_template(start:usize,template:&str, tree: &Vec<PositionTree>)-> Option<Vec<ExpressionNode>> {
    if template.is_empty() {
        return None;
    }
    let  template_len = template.len();
    let mut expression_list = vec![];

    let mut read_index = start;
    for position in tree {
        let start = position.start;
        let end = position.end;
        let start_name = &position.start_name;
        let end_name = &position.end_name;
        let end_name_len = end_name.len();
        let tag_child = &position.child;
        let tag_branch = &position.branch;
        let child_last_end = position.child_last_end;
        // if let Some(child_last_end) = child_last_end {
        //     read_index = child_last_end;
        // }

        // println!("----------------------------------------------{} -- {:?}", read_index,position);
        println!("----------------------------------------------{} ", read_index);
        if  read_index < start {
            let text =    &template[read_index ..start];
            if let Some(text_node) =text_node::new_node(text) {
                expression_list.push(text_node);
            }
            println!("first-tag_first {:?} start:{:?} end:{:?} node:{:?}", text,read_index,start,position);
        }

        println!("{} {} start_name:{} end_name:{}", start, end,start_name,end_name);
        let tag_text = &template[start..end];
        println!("tag_text:{:?}", tag_text);

        println!("tag_child:{:?}", tag_child);

        read_index = end+end_name_len;

        let mut child_node_list: Option<Vec<ExpressionNode>> = None;
        if let Some(pos) = tag_text.find(')') {
            let child_start = start + pos + 1;
            let child_end = end;
            // println!("tag_start:{} tag_end:{}",tag_start,tag_end);
            println!("child_start:{:?} child_end:{}", child_start, child_end);
            // println!("total:{}", template.len());
            let child_text = &template[child_start..child_end];
            println!("child_text:{:?}", child_text);

            // read_index = child_end;

            child_node_list = tag_child
                .as_ref()
                .filter(|child| !child.is_empty())//如果不为空执行 and_then
                .and_then(|child| parse_template(child_start,template, &child))
                .or_else(|| text_node::new_node(child_text).map(|text_node| vec![text_node]));
        }




        let mut branch_list = vec![];
        if let Some(branch  ) = tag_branch {
            if !branch.is_empty() {



                for else_tag in branch {
                    let start_name = &else_tag.start_name;
                    let end_name = &else_tag.end_name;
                    let end_name_len = end_name.len();
                    let else_tag_start = else_tag.start;
                    let else_tag_end = else_tag.end;
                    let else_child = &else_tag.child;
                    let child_last_end = else_tag.child_last_end;

                    println!("--child_last_end {:?}  read_index:{} {:?}",child_last_end,read_index ,else_tag);
                    if let Some(child_last_end) = child_last_end{

                    }


                    if start_name == "#elseif" {
                        let else_text = &template[else_tag_start..else_tag_end];

                        println!("else_tag-----------------------{:?}  else_text:{:?}", else_tag,else_text);

                        let mut branch_child_node_list:Option<Vec<ExpressionNode>> = None;
                        if let Some(pos) = else_text.find(')') {
                            let child_start = else_tag_start+pos+1;
                            let child_end = else_tag_end ;
                            let child_text = &template[child_start..child_end];

                            read_index = child_end+end_name_len;

                            println!("if_else_child_start:{:?} if_else_child_end:{} if_else_child_text:{:?}" , child_start,child_end,child_text);
                            branch_child_node_list = else_child
                                .as_ref()
                                .filter(|child| !child.is_empty())//如果不为空执行 and_then
                                .and_then(|child| parse_template(read_index,template, &child))
                                .or_else(|| text_node::new_node(child_text).map(|text_node| vec![text_node]));
                        }

                        let condition = get_if_condition(else_text);
                        if let Some(condition) = condition {
                            branch_list.push(ExpressionNode::IfNode {
                                condition: condition.parse().unwrap(),
                                children: branch_child_node_list,
                                else_list: None
                            });
                        }

                    }else if start_name == "#else" {

                        let mut branch_child_node_list:Option<Vec<ExpressionNode>> = None;
                        let pos = start_name.len();
                        let child_start = else_tag_start+pos;
                        let child_end = else_tag_end ;

                        let child_text = &template[child_start..child_end];
                        println!("else_child_start:{:?} else_child_end:{} child_text:{:?}", child_start,child_end,child_text);

                        read_index = child_end+end_name_len;


                        branch_child_node_list = else_child
                            .as_ref()
                            .filter(|child| !child.is_empty())//如果不为空执行 and_then
                            .and_then(|child| parse_template(read_index,template, &child))
                            .or_else(|| text_node::new_node(child_text).map(|text_node| vec![text_node]));

                        branch_list.push(ExpressionNode::IfNode {
                            condition: "true".to_string(),
                            children: branch_child_node_list,
                            else_list: None
                        });

                    }
                }
            }
        }





        println!("-------------------------------------------{:?}", start_name);
        if start_name == "#if" {

            let condition = get_if_condition(tag_text);
            println!("-----------------------------------------condition:{:?}",condition);
            if let Some(condition) = condition {
                expression_list.push(ExpressionNode::IfNode {
                    condition: condition.parse().unwrap(),
                    children: child_node_list,
                    else_list: Some(branch_list)
                });
            }

        }else if start_name == "#foreach" {
            let condition = crate::tag::tag_parse::get_if_condition(tag_text);
            if let Some(condition) = condition {
                if let Some((left, right)) = get_foreach_condition(condition) {
                    expression_list.push(ExpressionNode::ForeachNode {
                        collection: left,
                        element: right,
                        children: child_node_list,
                    });
                }
            }
        }


        // read_index = end;

    }


    Some(expression_list)
}


pub fn get_if_condition(input: &str) -> Option<&str> {
    // 查找 'if' 后面的 '(' 和第一个 ')'
    if let Some(start) = input.find('(') {
        if let Some(end) = input[start..].find(')') {
            // 返回括号内的内容
            return Some(&input[start + 1..start + end]);
        }
    }
    None
}

fn get_foreach_condition(input: &str) -> Option<(String, String)> {
    // 去掉字符串两端的空白字符
    let trimmed_input = input.trim();
    // 查找 'in' 的位置
    if let Some(in_index) = trimmed_input.find("in") {
        // 提取 'in' 之前和之后的内容
        let left = trimmed_input[..in_index].trim().to_string();
        let right = trimmed_input[in_index + 2..].trim().to_string();

        return Some((left, right));
    }

    None
}