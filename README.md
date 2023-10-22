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
