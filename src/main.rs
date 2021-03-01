// use std::io;
use std::env;
use std::fs::File;
use std::io::prelude::*;

mod scanner;
mod types;
// TODO option for running a interpreter prompt

fn extract_contents(s: String) -> std::io::Result<String> {
    let mut file = File::open(s)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// struct InterpreterError(bool);

// book describes a seperate reporter, and a flag, will
// come back and add them later as I need it.
#[allow(dead_code)]
fn error(line: usize, message: String) {
    println!("[ line: {} ] Error: {}", line, message);
}

fn main() {
    let arg_list: Vec<String> = env::args().collect();
    match arg_list.len() {
        1 => println!("Looking for a script file"),
        2 => match extract_contents(arg_list[1].clone()) {
            Ok(v) => scanner::Scanner::new(v).scan_file(),
            Err(e) => println!("err: {}", e),
        },
        _ => println!("I'm assuming something went wrong"),
    }
}
