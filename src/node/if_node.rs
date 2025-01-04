use crate::node::node_parse::ExpressionNode;
use crate::tag::tag_parse::TagFinalPosition;

pub fn new_node(tag:&TagFinalPosition, child_node_list:Option<Vec<ExpressionNode>>) -> Option<ExpressionNode>{
    println!("if_node tag: {:?} child_node_list:{:?}", tag,child_node_list);

    None
}