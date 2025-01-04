use crate::node::node_parse::ExpressionNode;

pub fn new_node_trim(text: &str) -> ExpressionNode {
    ExpressionNode::TextNode {
        text: text.trim().to_string(),
    }
}


pub fn new_node(text: &str) -> ExpressionNode {
    if is_wrapped_with_newlines(text) && text.len()>1 {
        return ExpressionNode::TextNode {
            text:  remove_single_leading_newline(text).to_string(),
        };
    }
    ExpressionNode::TextNode {
        text: text.to_string(),
    }
}



fn is_wrapped_with_newlines(input: &str) -> bool {
    input.starts_with('\n') && input.ends_with('\n')
}


fn remove_single_leading_newline(input: &str) -> &str {
    if input.starts_with('\n') {
        &input[1..]
    } else {
        input
    }
}
