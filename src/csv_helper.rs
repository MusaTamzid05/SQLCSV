use csv;
use std::collections::HashMap;
use std::error::Error;

type Record = HashMap<String , String>;


pub fn count(colname : &str , filepath :&str)-> Result<() , Box<Error>> {
    let mut rdr = csv::Reader::from_path(filepath)?;
    let mut sum = 0;

    for result in rdr.deserialize() {
        let record : Record = result?;

        if record[colname].len() == 0 {
            continue;
        }
        sum += 1;
        //println!("{}" , record[colname]);
    }


    Ok((println!("Count : {}" , sum)))
}
