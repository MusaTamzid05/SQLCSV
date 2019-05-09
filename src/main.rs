extern crate regex;

mod sql_parser;
use std::process;

fn main() {


    let sql = "SELECT COUNT(Data)";

    if !sql_parser::is_valid(sql) {
        println!("Its not a valid sql");
        process::exit(1);
    } 

    let colname : &str = sql_parser::get_column(sql);
}
