extern crate regex;

mod sql_parser;
mod csv_helper;
use std::process;
use std::env;

fn main() {


    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: 'csv path' 'SQL'");
        process::exit(1);
    }

    let path = &args[1];
    let sql =  &args[2];

    if !sql_parser::is_valid(sql) {
        println!("Its not a valid sql");
        process::exit(2);
    } 

    let colname : &str = sql_parser::get_column(sql);


    if sql.contains("COUNT") {
        csv_helper::count(colname , path);
    } else if sql.contains("AVG") {
        csv_helper::avg(colname , path);
    } else if sql.contains("MAX") {
        csv_helper::max(colname , path);
    } else if sql.contains("MIN") {
        csv_helper::min(colname , path);
    } else {
        println!("Invalid");
    }

}
