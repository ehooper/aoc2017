extern crate libc;

pub fn is_tty() -> bool {
    unsafe { libc::isatty(libc::STDIN_FILENO) != 0 }
}

pub fn prompt() {
    use std::io::Write;

    print!("> ");
    std::io::stdout().flush().unwrap();
}
