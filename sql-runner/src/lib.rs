use rusqlite::{Connection, Result};
use csv::ReaderBuilder;
use std::error::Error;



pub fn create_db() -> Result<()> {
    let conn = Connection::open("flower.db")?;

    conn.execute(
        "create table if not exists iris_info (
             sepal_length numeric not null,
             sepal_width numeric not null,
             petal_length numeric not null,
             petal_width numeric not null,
             species text not null
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


    while let Some(result) = reader.records().next() {
        let record = result?;

        let s_length = &record[0];
        let s_width = &record[1];
        let p_length = &record[2];
        let p_width = &record[3];
        let species = &record[4];

        if let Err(err) = conn.execute(
            "INSERT INTO iris_info (sepal_length, sepal_width, petal_length, petal_width, species) values (?1, ?2, ?3, ?4, ?5)",
            &[&s_length, &s_width, &p_length, &p_width, &species],
        ) {
            eprintln!("Error inserting row: {}", err);
        }

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