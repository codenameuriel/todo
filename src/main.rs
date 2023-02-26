use std::fs::{OpenOptions, File};
use std::io::{BufRead, BufReader, prelude::*};

use clap::{Command, Arg, ArgAction};

fn main() {
    // todo file to store todos
    let mut todo_file = OpenOptions::new()
        .read(true)
        .create(true) // create the file if it doesn't exist
        .append(true) // don't overwrite existing content
        .open("todo.txt").unwrap();

    // define cli args
    let arg_matches = Command::new("Todo")
        .about("Program to manage todos")
        .args([
            Arg::new("list")
                .short('l')
                .long("list")
                .help("List all current todos")
                .action(ArgAction::SetTrue), // bool
            Arg::new("add")
                .short('a')
                .long("add")
                .value_name("TODO")
                .help("Add a todo")
                .action(ArgAction::Set), // Option<&T>
            Arg::new("delete")
                .short('d')
                .long("delete")
                .value_name("LINE NUMBER")
                .help("Delete a todo")
                .action(ArgAction::Set) // Option<&T>
        ])
        .get_matches();

    // handle -l --list flags
    if arg_matches.get_flag("list") {
        list(&todo_file).unwrap();
    }

    // handle -a --add flags
    let add_arg_val: Option<&String> = arg_matches.get_one("add");

    if let Some(todo) = add_arg_val {
        println!("todo: \"{}\" was added!", todo);
        write(&mut todo_file, todo).unwrap();
    }

    // handle -d --delete flags
    let delete_arg_val: Option<&String> = arg_matches.get_one("delete");

    if let Some(line_number) = delete_arg_val {
        println!("todo line number: {}", line_number);
        delete(&mut todo_file, line_number).unwrap();
    }
}

fn list(file: &File) -> Result<(), std::io::Error> {
    let reader = BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        println!("{}: {}", i + 1, line?);
    }

    Ok(())
}

fn write(file: &mut File, todo: &String) -> Result<(), std::io::Error> {
    let mut todo = todo.clone();
    todo.push_str("\n");
    file.write_all(todo.as_bytes())?;
    file.flush()?;
    Ok(()) // no issues
}

fn delete(file: &mut File, line_num: &String) -> Result<(), Box<dyn std::error::Error>> {
    let line_num: usize = line_num.parse()?;

    let mut todos = String::new();

    let reader = BufReader::new(file.try_clone()?);

    for (i, line) in reader.lines().enumerate() {
        if i + 1 != line_num {
            // println!("line: {}", line.unwrap());
            let line = line.unwrap();
            todos.push_str(&line);
            todos.push_str("\n");
        }
    }

    // clear file first
    file.set_len(0)?;
    file.flush()?;

    // write todos
    file.write_all(todos.as_bytes())?;
    file.flush()?;

    Ok(())
}
