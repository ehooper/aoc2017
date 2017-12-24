extern crate libc;

use std::io;
use std::io::prelude::*;

pub fn is_tty() -> bool {
    unsafe { libc::isatty(libc::STDIN_FILENO) != 0 }
}

pub fn prompt() {
    print!("> ");
    io::stdout().flush().unwrap();
}

pub fn run_lines<F>(run : F) where F : Fn(&str) {
    let mut input = String::new();

    if ! is_tty() {
        if io::stdin().read_to_string(&mut input).is_ok() {
            run(&input);
        }
        return;
    }

    prompt();
    while io::stdin().read_line(&mut input).is_ok() {
        run(input.trim_right());
        input.clear();
        prompt();
    }
}

pub fn get_multiline(mut input : &mut String) -> io::Result<usize> {
    if ! is_tty() {
        return io::stdin().read_to_string(&mut input);
    }
    let mut buffer = String::new();
    loop {
        io::stdin().read_line(&mut buffer)?;
        if buffer.trim_right().is_empty() {
            return Ok(input.len());
        }
        input.push_str(&buffer);
        buffer.clear();
    }
}

pub fn run_multiline<F>(prompt : &str, run : F) where F : Fn(&str) {
    let mut input = String::new();

    if ! is_tty() {
        if io::stdin().read_to_string(&mut input).is_ok() {
            run(&input);
        }
        return;
    }

    let mut buffer = String::new();
    println!("{}", prompt);
    while io::stdin().read_line(&mut buffer).is_ok() {
        if buffer.trim_right().is_empty() {
            run(&input);
            input.clear();
            println!();
            println!("{}", prompt);
        } else {
            input += &buffer;
        }
        buffer.clear();
    }
}
