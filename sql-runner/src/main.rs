use csv::ReaderBuilder;
use std::error::Error;
use sql_runner::{create_db, fill_data, use_query};

pub fn read_csv() -> Result<(), Box<dyn Error>> {

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path("../iris.csv")?;

    for result in reader.records(){
        let record = result?;

        let column0 = &record[0];
        let column1 = &record[1];
        
        println!("Column 0 {} and Column 1 {}", column0, column1);
    }

    Ok(())
}


fn main() {
    println!("I am here");
    let _ = read_csv();
    println!("I am done");
}
