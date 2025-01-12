use regex::Regex;

#[test]
pub fn test(){
    let input = "#if( true) true #end";

    let re = Regex::new(r"(?s)#if\s*\(\s*([^()]*(\([^()]*\))*[^()]*)\)").unwrap();

    if let Some(captures) = re.captures(input) {
        if let Some(condition) = captures.get(1) {
            println!("-------------condition:{:?}",condition.as_str());

        }
    }

}

