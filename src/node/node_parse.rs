use std::collections::HashMap;
use serde_json::Value;
use crate::node::{foreach_node, if_node, text_node};
use crate::tag::tag_parse::TagFinalPosition;

#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionNode {
    TextNode {
        text: String,
    },
    IfNode {
        condition: String,
        children: Option<Vec<ExpressionNode>>,
    },
    ForeachNode {
        collection: String,
        element: String,
        children: Option<Vec<ExpressionNode>>,
    },
}

impl ExpressionNode {

    pub fn create_fixed<F>(creator: F) -> Option<ExpressionNode>
    where
        F: FnOnce(TagFinalPosition,Option<Vec<ExpressionNode>>) -> Option<ExpressionNode>,
    {

        creator(TagFinalPosition{
            tag: "".to_string(),
            start: 0,
            end: 0,
            child: None,
        }, None)
    }

    pub fn new_node(tag:&TagFinalPosition, child_node_list:Option<Vec<ExpressionNode>>) -> Option<ExpressionNode> {
        if tag.tag == "#if" {
            return if_node::new_node(&tag,child_node_list);
        }else if tag.tag == "#foreach" {
            return foreach_node::new_node(&tag,child_node_list);
        }
        None
    }

    pub fn get_node_text(node:ExpressionNode,content: &HashMap<String, Value>) -> Option<String>{
        match node {
            ExpressionNode::TextNode { .. } => {
                text_node::node_to_string(&node, content)
            }
            ExpressionNode::IfNode { .. } => {
                if_node::node_to_string(&node, content)
            }
            ExpressionNode::ForeachNode { .. } => {
                foreach_node::node_to_string(&node, content)
            }
        }
    }

}
