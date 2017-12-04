use std::io;
use std::collections::hash_set::HashSet;

fn is_valid(passphrase : &str) -> bool {
    let mut hs = HashSet::<String>::new();
    for word in passphrase.split_whitespace() {
        if hs.insert(word.to_owned()) == false {
            return false;
        }
    }
    return true;
}

fn main() {
    let mut input = String::new();
    println!("Enter passphrase list:");
    let mut count : usize = 0;
    while let Ok(_) = io::stdin().read_line(&mut input) {
        if input.split_whitespace().count() == 0 {
            println!("{} valid", count);
            println!("Enter passphrase list:");
            count = 0;
        } else if is_valid(input.trim_right()) {
            count += 1;
        }
        input.clear();
    }
}
