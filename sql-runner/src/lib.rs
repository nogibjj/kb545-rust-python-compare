use rusqlite::{Connection, Result};
use csv::ReaderBuilder;
use std::error::Error;



pub fn create_db() -> Result<()> {
    let conn = Connection::open("flower.db")?;

    conn.execute(
        "create table if not exists iris_info (
             variety text primary key,
             petal_length numeric not null
         )",
        (),
    )?;

    Ok(())
}

pub fn fill_data() -> Result<(), Box<dyn Error>> {

    let conn = Connection::open("flower.db")?;

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path("../iris.csv")?;

    for result in reader.records(){
        let record = result?;

        let name = &record[2];
        let length = &record[4];
        
        conn.execute("INSERT INTO iris_info (variety, petal_length) values (?1, ?2)", 
            &[&name, &length],)?;
        
    }

    Ok(())
}

pub fn use_query(statement: String) -> Result<()>{
    let conn = Connection::open("flower.db")?;

    let mut stmt = conn.prepare(&statement.to_string())?;

    /*let _rows = stmt.query_map([], |row|{
        let variety: String = row.get(0)?;
        let petal_length: u32 = row.get(1)?;
        println!("Variety: {}, Petal_Length: {}", variety, petal_length);
        Ok(())
    })?;*/

    let mut rows = stmt.query([]).unwrap();

    while let Some(row) = rows.next()? {
        println!("{:?}", row);
    }

    Ok(())
}