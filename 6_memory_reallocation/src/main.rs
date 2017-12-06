use std::io;
use std::collections::HashMap;

type MemorySet = [u16];

fn redistribute(banks : &mut MemorySet) {
    let size = banks.len() as u16;
    if size == 0 {
        return;
    }
    let mut max = 0;
    let mut index = 0;
    for (i, &b) in banks.iter().enumerate() {
        if b > max {
            max = b;
            index = i;
        }
    }
    let fill = max / size;
    let mut extra = max % size;
    banks[index] = fill;
    let mut distribute = |start, end| {
        for b in banks[start..end].iter_mut() {
            *b += fill;
            if extra > 0 {
                *b += 1;
                extra -= 1;
            }
        }
    };
    distribute(index + 1, size as usize);
    distribute(0, index);
}

#[derive(Debug)]
struct Cycle {
    start : usize,
    length : usize,
}

fn find_cycle(banks : &mut MemorySet) -> Cycle {
    let mut seen: HashMap<Vec<u16>, usize> = HashMap::new();
    let mut count = 0;
    loop {
        match seen.insert(banks.to_vec(), count) {
            None => redistribute(banks),
            Some(step) => return Cycle{start: count, length: count - step},
        }
        count += 1;
    }
}

fn prompt() {
    use std::io::Write;

    print!("> ");
    io::stdout().flush().unwrap();
}

fn main() {
    let mut input = String::new();
    prompt();
    while let Ok(_) = io::stdin().read_line(&mut input) {
        let mut banks : Vec<u16> = input.trim_right().split_whitespace().map(|s| s.parse::<u16>().unwrap()).collect();
        println!("{:?}", find_cycle(&mut banks));
        input.clear();
        prompt();
    }
}
