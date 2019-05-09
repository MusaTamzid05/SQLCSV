use regex::Regex;

pub fn is_valid(pattern : &str) -> bool {

    let re = Regex::new(r"SELECT (COUNT|AVG|MAX|MIN)\(\w*\)").unwrap();

    if re.is_match(pattern) {
        return true;
    } 

    return false;

}
