use csv;
use std::collections::HashMap;
use std::error::Error;

type Record = HashMap<String , String>;


pub fn count(colname : &str , filepath :&str)-> Result<() , Box<Error>> {
    let mut rdr = csv::Reader::from_path(filepath)?;
    let mut current_count = 0;

    for result in rdr.deserialize() {
        let record : Record = result?;

        if record[colname].len() == 0 {
            continue;
        }
        current_count += 1;
    }


    Ok((println!("Count : {}" , current_count)))
}


pub fn avg(colname : &str , filepath :&str)-> Result<() , Box<Error>> {
    let mut rdr = csv::Reader::from_path(filepath)?;
    let mut sum = 0.0;
    let mut current_count = 0.0;

    for result in rdr.deserialize() {
        let record : Record = result?;

        if record[colname].len() == 0 {
            continue;
        }


        let value : f32 = record[colname].parse().unwrap();;
        sum +=  value;
        current_count += 1.0;
    }


    Ok((println!("Avg: {}" , sum / current_count)))
}


pub fn max(colname : &str , filepath :&str)-> Result<() , Box<Error>> {
    let mut rdr = csv::Reader::from_path(filepath)?;
    let mut max_value = -9999999999.00;

    for result in rdr.deserialize() {
        let record : Record = result?;

        if record[colname].len() == 0 {
            continue;
        }


        let value : f32 = record[colname].parse().unwrap();;

        if value >  max_value {
            max_value = value;
        }
    }


    Ok((println!("Max: {}" , max_value)))
}


pub fn min(colname : &str , filepath :&str)-> Result<() , Box<Error>> {
    let mut rdr = csv::Reader::from_path(filepath)?;
    let mut min_value = 9999999999.00;

    for result in rdr.deserialize() {
        let record : Record = result?;

        if record[colname].len() == 0 {
            continue;
        }


        let value : f32 = record[colname].parse().unwrap();;

        if value <  min_value{
            min_value = value;
        }
    }


    Ok((println!("Min : {}" , min_value)))
}
