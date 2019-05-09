use regex::Regex;

pub fn is_valid(sql: &str) -> bool {

    let re = Regex::new(r"SELECT (COUNT|AVG|MAX|MIN)\(\w*\)").unwrap();

    if re.is_match(sql) {
        return true;
    } 

    return false;
}

