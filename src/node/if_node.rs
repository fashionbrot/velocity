use std::collections::HashMap;
use serde_json::Value;
use crate::node::node_parse::ExpressionNode;
use crate::node::text_node;
use crate::tag::tag_parse::TagFinalPosition;
use crate::expression::expression_evaluator;

pub fn new_node(tag:&TagFinalPosition, child_node_list:Option<Vec<ExpressionNode>>) -> Option<ExpressionNode>{
    println!("if_node tag: {:?} child_node_list:{:?}", tag,child_node_list);

    None
}

pub fn node_to_string(node: &ExpressionNode,context:&HashMap<String, Value>) -> Option<String> {

    if let ExpressionNode::IfNode { condition,children,else_list } = node {
        let expression = text_node::normalize_variable_syntax(condition,context);
        println!("if expression:{:?}",expression);
        if let Ok(expression) = expression_evaluator::evaluate_expression(&expression) {
            if expression {

                println!("if expression:{:?}",expression);
                if let Some(child) = children{

                    let mut output = String::new();
                    for n in child {

                        let result = ExpressionNode::get_node_text(n.clone(),context);
                        if let Some(text) = result {
                            output.push_str(&text);
                        }
                    }
                    return Some(output);
                }

            }else {
                if let Some(else_list) = else_list{
                    for else_node in else_list {
                        if let ExpressionNode::IfNode {condition,children,else_list} = else_node {
                            let expression = text_node::normalize_variable_syntax(condition,context);
                            if let Ok(expression) = expression_evaluator::evaluate_expression(&expression) {
                                if expression {

                                    println!("if expression:{:?}",expression);
                                    if let Some(child) = children{

                                        let mut output = String::new();
                                        for n in child {

                                            let result = ExpressionNode::get_node_text(n.clone(),context);
                                            if let Some(text) = result {
                                                output.push_str(&text);
                                            }
                                        }
                                        return Some(output);
                                    }
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

// pub fn node_to_string(node: &ExpressionNode, context: &HashMap<String, Value>) -> Option<String> {
//     // 辅助函数：处理节点文本生成
//     fn process_children(children: &[ExpressionNode], context: &HashMap<String, Value>) -> String {
//         children.iter().filter_map(|n| ExpressionNode::get_node_text(n.clone(), context)).collect()
//     }
//
//     // 判断条件是否成立的封装函数
//     fn evaluate_condition(condition: &str, context: &HashMap<String, Value>) -> Option<bool> {
//         let expression = text_node::normalize_variable_syntax(condition, context);
//         if let Ok(expression) = expression_evaluator::evaluate_expression(&expression) {
//             Some(expression)
//         } else {
//             None
//         }
//     }
//
//     // 处理 IfNode 类型
//     if let ExpressionNode::IfNode { condition, children, else_list } = node {
//         if let Some(true) = evaluate_condition(condition, context) {
//             // 如果条件成立，处理 children
//             if let Some(children) = children {
//                 return Some(process_children(children, context));
//             }
//         }
//
//         // 如果条件不成立，检查 else_list 是否存在并进行处理
//         if let Some(else_list) = else_list {
//             for else_node in else_list {
//                 if let ExpressionNode::IfNode { condition, children, .. } = else_node {
//                     if let Some(true) = evaluate_condition(condition, context) {
//                         if let Some(children) = children {
//                             return Some(process_children(children, context));
//                         }
//                     }
//                 }
//             }
//         }
//     }
//     None
// }
