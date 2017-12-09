use std::io;

#[derive(Debug, Clone, Copy)]
enum StreamState {
    Normal,
    Garbage,
    Ignore
}

fn parse(stream : &str) -> (usize, usize) {
    use StreamState::*;

    let mut nest = 0;
    let mut score = 0;
    let mut garbage_count = 0;
    let mut state = Normal;
    for c in stream.chars() {
        match (state, c) {
            (Normal, '{')  => { nest += 1 }
            (Normal, '}')  => { score += nest; nest -= 1 }
            (Normal, '<')  => { state = Garbage }
            (Garbage, '>') => { state = Normal }
            (Garbage, '!') => { state = Ignore }
            (Garbage, _)   => { garbage_count += 1 }
            (Ignore, _)    => { state = Garbage }
            (_, _)         => { }
        }
    }
    (score, garbage_count)
}

fn prompt() {
    use io::Write;

    print!("> ");
    io::stdout().flush().unwrap();
}

fn main() {
    use std::env;
    use std::fs::File;
    use io::prelude::*;

    let mut input = String::new();

    if let Some(filename) = env::args_os().nth(1) {
        let mut file = File::open(filename).expect("file not found");
        file.read_to_string(&mut input).expect("error reading file");
        let (score, garbage) = parse(input.as_ref());
        println!("score: {}", score);
        println!("garbage: {}", garbage);
        return;
    }

    prompt();
    while let Ok(_) = io::stdin().read_line(&mut input) {
        let (score, garbage) = parse(input.trim_right());
        println!("score: {}", score);
        println!("garbage: {}", garbage);
        input.clear();
        prompt();
    }
}
