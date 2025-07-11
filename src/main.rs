use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Write, BufReader, BufRead, Seek, SeekFrom};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path: String = match args.get(1) {
        Some(arg) => {
            arg.clone()
        },
        None => panic!("Required argument")
    };

    let mut task_file = OpenOptions::new()
        .append(true)
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .expect("Error loading the file");

    println!("File already created, or initialized al succesfully loaded");
    println!("Welcome to Yisus FileSystem based: to-do");
    show_options();

    let mut option: String = String::new();
    io::stdin().read_line(&mut option).expect("Error with the input string");

    while ["1","2","3"].contains(&option.trim()) {
        match option.trim() {
            "1" => add_task(&mut task_file),
            "2" => list_task(&mut task_file),
            "3" => delete_task(&mut task_file),
            _ => break,
        }
        show_options();
        option.clear();
        io::stdin().read_line(&mut option).expect("Error with the input string");
    }
}

fn show_options() {
    println!("\n\nChoose what do you want to do now:");
    println!("1. Add task.");
    println!("2. List tasks.");
    println!("3. Remove task.");
    println!("Type any other key or number to exist, also you could use (Cmd + D)");
}

fn add_task(f: &mut File) {
    let mut task_name: String = String::new();
    println!("-----------------------------------------------------");
    println!("Input the task name: ");
    io::stdin().read_line(&mut task_name).expect("Error with the input string");
    writeln!(f, "{}", task_name.trim()).expect("Failed to write task");
    println!("-----------------------------------------------------");
}

fn list_task(f: &mut File) {
    println!("-----------------------------------------------------");
    println!("This is your current tasks: ");
    f.seek(SeekFrom::Start(0)).expect("Failed to seek to start");
    let reader = BufReader::new(f);
    for (index, line) in reader.lines().enumerate() {
        println!("{}.{}", index, line.unwrap());
    }
    println!("-----------------------------------------------------");
}

fn delete_task(_f: &mut File) {
    println!("Deleting task...");
}
