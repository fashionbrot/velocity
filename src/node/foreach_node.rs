use std::collections::HashMap;
use serde_json::Value;
use crate::node::ExpressionNode;
use crate::tag::tag_parse::TagFinalPosition;
use crate::token::token_parse::Tokenizer;

pub fn new_node(tag:&TagFinalPosition, child_node_list:Option<Vec<ExpressionNode>>) -> Option<ExpressionNode>{
    println!("foreach_node tag: {:?} child_node_list:{:?}", tag,child_node_list);
    let tag = &tag.tag;
    if tag == "#foreach" {

    }
    None
}

pub fn node_to_string(node: &ExpressionNode,context:&HashMap<String, Value>) -> Option<String> {

    None
}

