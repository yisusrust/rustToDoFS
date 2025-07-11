use std::fs::File;
use std::io::prelude::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path: String = match args.get(1) {
        Some(arg) => {
            arg.clone()
        },
        None => panic!("Required argument")
    };

    let task_file = File::open(path.clone());
    let task_file: File = match task_file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    File::create(path.clone()).expect("Failed to create file")
                },
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };

    println!("File already created, or initialized al succesfully loaded");
    println!("Welcome to Yisus FileSystem based: to-do");
    println!("Choose what do you want to do now:");
    show_options();

    let mut task_file = task_file;

    let _ = task_file.write_all(b"Hello world");
}

fn show_options() {
    println!("1. Add task.");
    println!("2. List tasks.");
    println!("3. Remove task.");
    println!("Type any other key or number to exist, also you could use (Cmd + D)");
}
