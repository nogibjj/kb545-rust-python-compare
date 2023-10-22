use sql_runner::{create_db, fill_data, use_query};
use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Your Name <your.email@example.com>",
    about = "Query to Execute"
)]

struct Opts {
    #[clap(long)]
    query: String,
}

/*pub fn read_csv() -> Result<(), Box<dyn Error>> {

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
}*/


fn main() {
    let opts: Opts = Opts::parse();

    let query = opts.query;

    let _ = create_db();
    let _ = fill_data();
    let _ = use_query(query);
}
