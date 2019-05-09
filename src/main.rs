extern crate regex;

mod sql_parser;

fn main() {
    
    if sql_parser::is_valid("SELECT COUNT(Data)") {
        println!("Its a valid sql");
    } else {

        println!("Its a invalid sql");
    }
}
