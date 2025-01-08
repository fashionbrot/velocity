use std::fmt::Debug;
use regex::Match;
use serde::{Deserialize, Serialize};
use crate::tag::tag_parse::TagFinalPosition;

pub trait Node: Debug {
    fn tag_name(&self) -> &String;
    fn start(&self) -> usize;
    fn end(&self) -> usize;

    fn println(&self) {
        println!("Node Tag: {}, Start: {}, End: {}", self.tag_name(), self.start(), self.end());
    }
}

#[derive(Debug, Serialize, Deserialize,Clone,PartialEq)]
pub struct NodePosition {
    pub tag_name: String,
    pub tag_start:u32,
    pub tag_end :u32,
}

#[derive(Debug, Serialize, Deserialize,Clone,PartialEq)]
pub struct PairPosition{
    pub begin_name:String,
    pub begin:u32,
    // pub begin_start:u32,
    // pub begin_end:u32,
    pub finish_name:String,
    // pub finish_start:u32,
    // pub finish_end:u32,
    pub finish:u32,
    pub child:Option<Vec<PairPosition>>,
    pub branch:Option<Vec<PairPosition>>
}


impl PairPosition{
    pub fn new(begin:NodePosition, finish:NodePosition)->PairPosition{
        PairPosition{
            begin_name: begin.tag_name,
            // begin_start: begin.tag_start,
            // begin_end: begin.tag_end,
            begin: begin.tag_start,
            finish_name:finish.tag_name,
            // finish_start:finish.tag_start,
            // finish_end:finish.tag_end,
            finish: finish.tag_end,
            child: None,
            branch: None,
        }
    }

    pub fn is_child(&self, other: &PairPosition) -> bool {
        self.begin < other.begin && self.finish < other.finish
    }

    pub fn get_child(&self, list: &Vec<PairPosition>) -> Option<Vec<PairPosition>> {
        if list.is_empty() {
            return None;
        }

        let mut filtered: Vec<PairPosition> = list
            .iter()
            .filter(|p|  self.begin > p.begin &&  p.finish < self.finish )
            .cloned()
            .collect();

        if filtered.is_empty() {
            None
        } else {
            filtered.sort_by_key(|tag| tag.begin);
            Some(filtered)
        }
    }

    // pub fn is_root(&self, list: &Vec<PairPosition>) -> bool {
    //     if self.begin_name =="#else" || self.begin_name=="#elseif" {
    //         return false;
    //     }
    //     for x in list {
    //         if x.begin > self.begin && self.begin > x.begin{
    //             return false
    //         }
    //         if x.finish > self.finish && self.finish > x.finish {
    //             return false
    //         }
    //     }
    //     true
    // }

    pub fn is_root(&self, tags: &[PairPosition]) -> bool {
        if self.begin_name =="#else" || self.begin_name=="#elseif" {
            return false;
        }
        let begin = self.begin;
        let finish = self.finish - self.finish_name.len() as u32;
        for x in tags {
            if x.begin < begin && begin < x.finish{
                return false
            }
            if x.begin <finish && finish < x.finish {
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
            tag_start:tag_start as u32,
            tag_end:tag_end as u32,
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
pub struct TextNode {
    pub text: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IfNode {
    pub condition: String,
    pub position:PairPosition
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForeachNode {
    pub collection: String,
    pub element: String,
    pub position:PairPosition
}

#[derive(Clone, Debug, PartialEq)]
pub enum TreeNode {
    Text(TextNode),
    If(IfNode),
    Foreach(ForeachNode),
}



// impl Node for TextNode {
//     fn tag_name(&self) -> &String {
//         &self.tag_name
//     }
//     fn start(&self) -> usize {
//         self.start
//     }
//     fn end(&self) -> usize {
//         self.end
//     }
// }

// impl Node for IfNode {
//     fn tag_name(&self) -> &String {
//         &self.tag_name
//     }
//     fn start(&self) -> usize {
//         self.start
//     }
//     fn end(&self) -> usize {
//         self.end
//     }
// }
//
// impl Node for ForeachNode {
//     fn tag_name(&self) -> &String {
//         &self.tag_name
//     }
//     fn start(&self) -> usize {
//         self.start
//     }
//     fn end(&self) -> usize {
//         self.end
//     }
// }
