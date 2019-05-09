extern crate regex;

mod sql_parser;
mod csv_helper;
use std::process;

fn main() {


    let path = "./uspop.csv";
    let sql = "SELECT MAX(Population)";

    if !sql_parser::is_valid(sql) {
        println!("Its not a valid sql");
        process::exit(1);
    } 

    let colname : &str = sql_parser::get_column(sql);
    println!("{}" , colname);


    if sql.contains("COUNT") {
        csv_helper::count(colname , path);
    } else if sql.contains("AVG") {
        csv_helper::avg(colname , path);

    } else if sql.contains("MAX") {
        csv_helper::max(colname , path);

    }

}
