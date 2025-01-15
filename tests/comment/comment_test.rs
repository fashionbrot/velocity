use crate::log_config;
use lazy_static::lazy_static;
use regex::Regex;
use velocity::{read_file, render_default, render_default_path};

#[test]
pub fn marker_test() {
    log_config::print_debug_log();

    let template_path = "tests/comment/comment.vm";
    let template = if let Ok(content) = read_file(template_path) {
        content // 直接将 String 赋值给 template
    } else {
        String::new() // 返回一个空字符串作为默认值
    };

    println!("template: {}", template);

    let result = remove_between_markers(&template);
    println!("{}", result);
}

fn find_tag_bracket_range(input: &str, tag: &str) -> Option<(usize, usize)> {
    let start_index = match input.find(tag) {
        Some(index) => index,
        None => return None,
    };
    let end_tag = match tag {
        "#*" => "*#",
        "*#" => "#*",
        _ => return None,
    };
    let end_index = input[start_index + tag.len()..].find(end_tag);
    if let Some(end_index) = end_index {
        Some((
            start_index,
            start_index + tag.len() + end_index + end_tag.len(),
        ))
    } else {
        None
    }
}

fn remove_between_markers(input: &str) -> String {
    let mut result = String::new();
    let mut skip = 0;
    let mut index = 0;
    while index < input.len() {
        if let Some((start_index, end_index)) = find_tag_bracket_range(&input[skip..], "#*") {
            result.push_str(&input[skip..start_index]);
            skip = start_index + (end_index - start_index);
        } else {
            result.push_str(&input[skip..]);
            break;
        }
        index = skip;
    }
    result
}

#[test]
pub fn test2() {
    log_config::print_debug_log();

    let template_path = "tests/comment/comment.vm";
    let template = if let Ok(content) = read_file(template_path) {
        content // 直接将 String 赋值给 template
    } else {
        String::new() // 返回一个空字符串作为默认值
    };
    println!("template: {}", template);

    let result = render_default(template.as_str());
    if let Ok(content) = result {
        println!("----------------------------------------------------------------\n{}", content);
        println!("----------------------------------------------------------------")
    }

}

#[test]
pub fn test22() {

    let result = render_default_path("tests/comment/comment.vm");
    if let Ok(content) = result {
        println!("----------------------------------------------------------------\n{}", content);
        println!("----------------------------------------------------------------")
    }
}

lazy_static! {
    // 块注释 #* ... *#
    static ref BLOCK_COMMENT_RE: Regex = Regex::new(r"#\*.*?\*#").unwrap();
    // 匹配行尾注释 ## 开头到行尾（包括换行符）
    static ref LINE_COMMENT_RE: Regex = Regex::new(r"(?m)^##.*\n?").unwrap(); // 匹配整行 ## 开头的注释
}

fn remove_velocity_comments(template: &str) -> String {
    // 移除块注释（#* ... *#）
    let template = BLOCK_COMMENT_RE.replace_all(template, "");

    // 移除整行以 ## 开头的注释，包括换行符
    let template = LINE_COMMENT_RE.replace_all(&template, "");
    // 返回处理后的字符串
    template.to_string()
}

#[test]
pub fn test3() {
    let template = "123 ##这是一段注释\r\n好好\r\n这是另一行\r\n";

    // 定义正则表达式，匹配 \r\n
    let tags_pattern = r"\r\n";  // 直接匹配 \r\n

    // 创建正则表达式
    let re = Regex::new(tags_pattern).unwrap();

    // 使用 find 获取第一个匹配项
    if let Some(capture) = re.find(template) {
        // 获取第一个匹配项的起始字节下标
        let index = capture.start();
        println!("Found match at byte index {}: {}", index, capture.as_str());
    } else {
        println!("No match found.");
    }

    println!("{:?}", find_char_index(template, "\r\n"));

    println!("{:?}", &template[4..24+2])
}

fn find_char_index(input: &str, target: &str) -> Option<usize> {
    if target.is_empty() {
        return None;
    }
    let re = Regex::new(format!(r"{}",target).as_str()).unwrap();
    if let Some(capture) = re.find(input) {
        return Some(capture.start());
    }
    None
}