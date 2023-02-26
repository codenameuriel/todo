use std::fs::{OpenOptions, File};
use std::io::{BufRead, BufReader, prelude::*};

use clap::{Command, Arg, ArgAction, ArgMatches};

fn main() {
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

    // todo file to store todos
    let mut todo_file = OpenOptions::new()
        .read(true)
        .create(true) // create the file if it doesn't exist
        .append(true) // don't overwrite existing content
        .open("todo.txt").unwrap();

    on_list(&todo_file, &arg_matches).unwrap();

    on_add(&mut todo_file, &arg_matches).unwrap();

    on_delete(&mut todo_file, &arg_matches).unwrap();
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
            let line = line?;
            todos.push_str(&line);
            todos.push_str("\n");
        } else {
            println!("deleted todo: {}", line?);
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

fn on_list(todo_file: &File, arg_matches: &ArgMatches) -> Result<(), std::io::Error> {
    // handle -l --list flags
    if arg_matches.get_flag("list") {
        list(&todo_file)?;
    }

    Ok(())
}

fn on_add(todo_file: &mut File, arg_matches: &ArgMatches) -> Result<(), std::io::Error> {
    // handle -a --add flags
    let add_arg_val: Option<&String> = arg_matches.get_one("add");

    if let Some(todo) = add_arg_val {
        println!("todo: \"{}\" was added!", todo);
        write(todo_file, todo)?;
    }

    Ok(())
}

fn on_delete(todo_file: &mut File, arg_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    // handle -d --delete flags
    let delete_arg_val: Option<&String> = arg_matches.get_one("delete");

    if let Some(line_number) = delete_arg_val {
        delete(todo_file, line_number)?;
    }

    Ok(())
}