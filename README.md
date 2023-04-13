# wk11-mini-rust-jl1188:
A Rust Mini Project command-line tool that counts the difference between two given dates. 
Dates are given in the format 'YYYY-MM-DD'.

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* [rust-new-project-template](https://github.com/noahgift/rust-new-project-template)
* [rust-marco-polo-example](https://github.com/noahgift/rust-mlops-template/tree/main/MarcoPolo)

## Usage
<code>cargo run -- days --first "2023-01-11" --second "2023-05-06"</code>
 
 The command line tool uses the subcommand "<code>days</code>" and takes in two arguments, a first <code>String</code> named "<code>first</code>," and a second <code>String</code> named "<code>second</code>". The result should be outputted as "Number of days between 2023-01-11 and 2023-05-06 is: 115". 


```
     Running `target/debug/days_between days --first 2023-01-11 --second 2023-05-06`
Number of days between 2023-01-11 and 2023-05-06 is: 115
```