extern crate libc;

pub fn is_tty() -> bool {
    unsafe { libc::isatty(libc::STDIN_FILENO) != 0 }
}

pub fn prompt() {
    use std::io::Write;

    print!("> ");
    std::io::stdout().flush().unwrap();
}

pub fn run_lines<F>(run : F) where F : Fn(&str) {
    use std::io::Read;

    let mut input = String::new();

    if ! is_tty() {
        if std::io::stdin().read_to_string(&mut input).is_ok() {
            run(&input);
        }
        return;
    }

    prompt();
    while std::io::stdin().read_line(&mut input).is_ok() {
        run(input.trim_right());
        input.clear();
        prompt();
    }
}

pub fn run_multiline<F>(prompt : &str, run : F) where F : Fn(&str) {
    use std::io::Read;

    let mut input = String::new();

    if ! is_tty() {
        if std::io::stdin().read_to_string(&mut input).is_ok() {
            run(input.trim_right());
        }
        return;
    }

    let mut buffer = String::new();
    println!("{}", prompt);
    while std::io::stdin().read_line(&mut buffer).is_ok() {
        if buffer.trim().is_empty() {
            run(input.trim_right());
            input.clear();
            println!("{}", prompt);
            println!();
        }
        input += &buffer;
        buffer.clear();
    }
}
