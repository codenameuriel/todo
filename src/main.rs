use std::fs::OpenOptions;
use std::io::prelude::*;
use std::env;

fn main() {
    let mut args = env::args();
    args.next().unwrap();
    let todo = args.next().unwrap();
    
    // builder pattern
    let mut todo_file = OpenOptions::new()
        .create(true) // create the file if it doesn't exist
        .append(true) // don't overwrite existing content
        .open("todo.txt").unwrap();

    todo_file.write_all(todo.as_bytes()).unwrap();
    todo_file.flush().unwrap();
}
