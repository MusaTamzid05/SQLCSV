use regex::Regex;

pub fn is_valid(sql: &str) -> bool {

    let re = Regex::new(r"SELECT (COUNT|AVG|MAX|MIN)\(\w*\)").unwrap();

    if re.is_match(sql) {
        return true;
    } 

    return false;
}

pub fn get_column(sql : &str) -> &str {

    // TODO : its a hack , find a better way ! maybe using regex.
    let vec_data : Vec<&str> = sql.split('(').collect();
    let col_data : Vec<&str> = vec_data[vec_data.len() -1].split(')').collect();

    return col_data[0];

}
