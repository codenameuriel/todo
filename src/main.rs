use std::fs::OpenOptions;
// use std::io::prelude::*;
// use std::io::ErrorKind;

fn main() {
    // mock user input
    let todo = "item one on todo";

    // open a file, if it does not exist, create it
    // then write to it
    // handle errors

    // let todo_file = File::open("todo.txt").unwrap_or_else(|e| {
    //     if e.kind() == ErrorKind::NotFound {
    //         File::create("todo.txt").expect("Problem creating todo.txt...")
    //     } else {
    //         panic!("Problem opening todo.txt...");
    //     }
    // });

    // builder pattern
    let todo_file = OpenOptions::new()
        .create(true) // create the file if it doesn't exist
        .append(true) // don't overwrite existing content
        .open("todo.txt").unwrap();
}
