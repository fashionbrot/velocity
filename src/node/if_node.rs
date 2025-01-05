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

    if let ExpressionNode::IfNode { condition,children } = node {
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

            }
        }
    }
    None
}