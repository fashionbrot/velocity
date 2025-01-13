use regex::Regex;

#[test]
pub fn test(){
    let input = "#if   ( true &&  (true && (true)) || (1==1 && 我是你大爷 (true || (2==2 && true)) && true ) && (1==2))()(内容括号)#end";
    for x in input.chars().enumerate(){
        println!("{} {}",x.1,x.0 );
    }
    if let Some((first, end)) = find_tag_bracket_range(input, "#if") {
        let value =  input[first..end+1].trim();
        println!("{:?}",value);
        assert_eq!(value,r#"( true &&  (true && (true)) || (1==1 && 我是你大爷 (true || (2==2 && true)) && true ) && (1==2))"#)
    } else {
        assert!(false);
    }

}


pub fn find_tag_bracket_range(input: &str, tag: &str) -> Option<(usize, usize)> {
    let start_index = match input.find(tag) {
        Some(index) => index,
        None => return None,
    };

    let mut stack = 0;
    let mut first_open_paren_index = None;

    for (i, c) in input[start_index..].char_indices() {
        match c {
            '(' => {
                if first_open_paren_index.is_none() {
                    first_open_paren_index = Some(start_index + i);
                }
                stack += 1;
            }
            ')' => {
                stack -= 1;
                if stack == 0 {
                    return Some((first_open_paren_index?, start_index + i));
                }
            }
            _ => {}
        }
    }
    None
}


#[test]
pub fn test_tag(){

    let input = r#"123#set($name="我用set修改了name" ) 456"#;
    if let Some(end_index) = find_tag_end(input,"#set") {
        let value =  input[0..end_index+1].trim();
        println!("{:?}",value);
        assert_eq!(value,r#"123#set($name="我用set修改了name" )"#)
    } else {
        assert!(false);
    }
}

fn find_tag_end(input: &str, tag: &str) -> Option<usize> {
    let start_index = match input.find(tag) {
        Some(index) => index,
        None => return None,
    };
    let mut stack = 0;
    for (i, c) in input[start_index..].char_indices() {
        match c {
            '(' => stack += 1,
            ')' => {
                stack -= 1;
                if stack == 0 {
                    return Some(start_index + i);
                }
            }
            _ => {}
        }
    }
    None
}



// fn main() {
//     let input = "#if( true &&  (true && (true)) || (1==1 && (true || (2==2 && true)) ))()(内容括号)#end";
//     if let Some((first, end)) = find_tag_end(input, "#if") {
//         println!("The first ( is at index: {}", first);
//         println!("The end of #if expression is at index: {}", end);
//     } else {
//         println!("No valid end found for #if expression");
//     }
// }
