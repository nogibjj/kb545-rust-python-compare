[![Build binary release](https://github.com/nogibjj/kb545-rust-python-compare/actions/workflows/release.yml/badge.svg)](https://github.com/nogibjj/kb545-rust-python-compare/actions/workflows/release.yml)
[![Lints](https://github.com/nogibjj/kb545-rust-python-compare/actions/workflows/lint.yml/badge.svg)](https://github.com/nogibjj/kb545-rust-python-compare/actions/workflows/lint.yml)
[![Format](https://github.com/nogibjj/kb545-rust-python-compare/actions/workflows/rustfmt.yml/badge.svg)](https://github.com/nogibjj/kb545-rust-python-compare/actions/workflows/rustfmt.yml)
[![Tests](https://github.com/nogibjj/kb545-rust-python-compare/actions/workflows/tests.yml/badge.svg)](https://github.com/nogibjj/kb545-rust-python-compare/actions/workflows/tests.yml)

# Mini-Project 8: Rust Python Comparison

In this project, we took a Python script that runs a query on an SQLite database, and completely rewrote it in Rust. Then, in order to highlight improvements, we recorded how fast it took for both entire scripts to execute. The results are shown below.

# What both scripts do
Both scripts create an SQLite database. Then, both will query from the database and output the results. The following below are the different execution times.

### Rust Timing
![Alt text](image.png)

### Python Timing
![image](https://github.com/nogibjj/kb545-rust-python-compare/assets/55768636/8e198933-beb4-4864-8cfd-a0f3e89ebf7b)

### How to Run the Code

In order to run this code, you need to execute the following command.

``` cargo run -- --query "ENTER YOUR QUERY HERE" ```

This will create the database locally, fill the values, and then execute any query that was passed in. The one rule is that the query needs to be from the ```iris_info``` table

### How the Rust Scipt was Written
The Rust script was written by utilizing the ```rusqlite``` package for creating the SQLite Database and executing the query, and utilizing the ```csv``` package for reading in the csv values before populating the SQLite Database. The crate to output the timing of the script was ```std::time```, and the CLI tool used was ```clap```. All of these have their respective dependencies added to the Cargo.toml file.

### Tests

There are tests written to check that a DB is created in the end, as well as if the query executions successfully is ran. There are logging messages as well if the query execution fails for whatever reason.
