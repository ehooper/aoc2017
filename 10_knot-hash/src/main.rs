extern crate util;
extern crate knot_hash;

use util::{prompt, is_tty};
use knot_hash::*;

fn run_part1() {
    let mut input = String::new();

    macro_rules! reset {
        () => { input.clear(); prompt(); }
    }

    let mut knot_hash = KnotHash::new();

    prompt();
    while std::io::stdin().read_line(&mut input).is_ok() {
        let lengths : Result<Vec<u8>, _> = input.split(|c : char| c == ',' || c.is_whitespace())
            .filter(|s| ! s.is_empty())
            .map(str::parse::<u8>)
            .collect();

        if let Err(e) = lengths {
            eprintln!("error parsing input: {} '{}'", std::error::Error::description(&e), input.trim_right());
            reset!();
            continue;
        }

        knot_hash.round(&lengths.unwrap(), &[]);
        {
            let byte_table = knot_hash.byte_table();
            let result = (byte_table[0] as usize) * (byte_table[1] as usize);
            println!("{}", result);
        }
        knot_hash.reset();

        reset!();
    }
}

fn run_part2() {
    use std::io::Read;

    let mut knot_hash = KnotHash::new();

    let mut input = String::new();
    if is_tty() {
        println!("Terminate input with EOF (Ctrl+D)");
        while std::io::stdin().read_to_string(&mut input).is_ok() {
            let digest = knot_hash.digest(input.trim_right().as_bytes());
            println!("{}", digest);
            input.clear();
        }
    } else if std::io::stdin().read_to_string(&mut input).is_ok() {
        let digest = knot_hash.digest(input.as_bytes());
        println!("{}", digest);
    }
}

fn main() {
    let arg = std::env::args().nth(1);
    let part2 = arg.map_or(false, |s| s == "--part2");

    if part2 {
        run_part2();
    } else {
        run_part1();
    }
}
