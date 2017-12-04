use std::io;
use std::collections::hash_set::HashSet;

// Assuming no more that 255 instances of the same character in a word.
#[derive(PartialEq, Eq, Hash)]
struct AnagramSet {
    count : [u8; 26],
}

impl<'a> From<&'a str> for AnagramSet {
    fn from(s : &'a str) -> Self {
        let mut aset = AnagramSet{count: [0; 26]};
        for &c in s.as_bytes() {
            if c >= b'a' && c <= b'z' {
                aset.count[(c - b'a') as usize] += 1;
            }
        }
        return aset;
    }
}

fn is_valid<'a, T : Eq + std::hash::Hash + From<&'a str>>(passphrase : &'a str) -> bool {
    let mut hs = HashSet::<T>::new();
    for word in passphrase.split_whitespace() {
        if hs.insert(T::from(word)) == false {
            return false;
        }
    }
    return true;
}

fn main() {
    let arg = std::env::args().nth(1);
    let no_anagrams = arg.map_or(false, |s| s == "--no-anagrams");

    let mut input = String::new();
    println!("Enter passphrase list:");
    let mut count : usize = 0;
    while let Ok(_) = io::stdin().read_line(&mut input) {
        if input.split_whitespace().count() == 0 {
            println!("{} valid", count);
            println!("Enter passphrase list:");
            count = 0;
        } else if no_anagrams {
            if is_valid::<AnagramSet>(input.as_ref()) {
                count += 1;
            }
        } else if is_valid::<String>(input.as_ref()) {
            count += 1;
        }
        input.clear();
    }
}
