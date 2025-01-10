use std::collections::HashMap;

pub mod node_parse;
pub mod if_node;
pub mod foreach_node;
pub mod text_node;

use std::fmt::Debug;
use lazy_static::lazy_static;
use regex::{escape, Match, Regex};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::tag::tag_parse::TagFinalPosition;

// 使用 lazy_static 宏定义静态变量 tags
lazy_static! {

    pub static ref TAGS: Vec<&'static str> = {
        let mut tags = Vec::new();
        tags.push("#if");
        tags.push("#elseif");
        tags.push("#else");
        tags.push("#foreach");
        tags.push("#set");
        tags.push("#end");
        tags
    };


        // 定义静态正则表达式模式，避免每次计算
    pub static ref TAGS_PATTERN: Regex = {
        // 生成正则表达式模式
        let pattern = TAGS.iter()
            .map(|tag| escape(*tag)) // 转义标签
            .collect::<Vec<String>>()
            .join("|"); // 使用 | 连接标签
        println!("pattern-------------{:?}" ,pattern);
        Regex::new(&format!(r"({})", pattern)).unwrap() // 返回正则表达式
    };
}


#[derive(Debug, Serialize, Deserialize,Clone,PartialEq)]
pub struct NodePosition {
    pub tag_name: String,
    pub tag_start:usize,
    pub tag_end :usize,
}

use std::fmt;

impl fmt::Display for NodePosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "NodePosition {{ tag_name: {}, tag_start: {}, tag_end: {} }}",
            self.tag_name, self.tag_start, self.tag_end
        )
    }
}

#[derive(Debug, Serialize, Deserialize,Clone,PartialEq)]
pub struct PositionTree {
    pub start_name:String,
    pub end_name:String,
    pub start:usize,
    pub end:usize,
    pub child_last_end: Option<usize>,
    pub child:Option<Vec<PositionTree>>,
    pub branch:Option<Vec<PositionTree>>,
}


impl PositionTree {
    pub fn new(begin:NodePosition, finish:NodePosition)-> PositionTree {
        PositionTree {
            start_name:begin.tag_name,
            end_name: finish.tag_name,
            start: begin.tag_start,
            end: finish.tag_start,
            child_last_end: None,
            child: None,
            branch: None,
        }
    }


    pub fn is_root(&self, tags: &[PositionTree]) -> bool {
        if self.start_name =="#else" || self.start_name=="#elseif" {
            return false;
        }
        let start = self.start;
        let end = self.end ;
        for x in tags {
            if x.start < start && start < x.end{
                return false
            }
            if x.start <end && end < x.end {
                return false
            }
        }
        true
    }


}

impl NodePosition {
    pub fn new(tag_name: &str, tag_start: usize, tag_end: usize) -> NodePosition {
        NodePosition{
            tag_name: tag_name.to_string(),
            tag_start:tag_start,
            tag_end:tag_end,
        }
    }

    pub fn build(capture: &Match) -> NodePosition {
        let tag_start = capture.start();
        let tag_end = capture.end();
        let tag_name = capture.as_str();
        NodePosition::new(tag_name, tag_start, tag_end)
    }

    pub fn print(&self) {
        println!("position tag_name: {:<10} tag_start:{:<10} tag_end:{:<10}", self.tag_name,self.tag_start,self.tag_end);
    }
}




#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionNode {
    TextNode {
        text: String,
    },
    IfNode {
        condition: String,
        children:  Option<Vec<ExpressionNode>>,
        else_list: Option<Vec<ExpressionNode>>,
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
            pre:None,
            child: None,
            else_list: None,
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


