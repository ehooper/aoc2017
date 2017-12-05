use std::io;
use std::io::Write;

fn check_consecutive(digits : &[u8]) -> Result<isize, String> {
    let mut sum : isize = 0;
    let iter = digits.iter().zip(digits.iter().cycle().skip(1));
    for (&d1, &d2) in iter {
        if d1 < b'0' || d1 > b'9' {
            return Err(format!("error: invalid digit '{}'", d1));
        }
        if d1 == d2 {
            sum += (d1 as isize) - (b'0' as isize);
        }
    }
    Ok(sum)
}

fn check_halfway_around(digits : &[u8]) -> Result<isize, String> {
    let mut sum : isize = 0;
    let nums : Vec<isize> = digits.iter().map(|d| (d - b'0') as isize).collect();
    let size = nums.len();
    for (i, &d1) in nums.iter().enumerate() {
        if d1 < 0 || d1 > 9 {
            return Err(format!("error: invalid digit '{}'", d1));
        }
        let d2 = nums[(i + size / 2) % size];
        if d1 == d2 {
            sum += d1;
        }
    }
    Ok(sum)
}

fn main() {
    let arg = std::env::args().nth(1);
    let part2 = arg.map_or(false, |s| s == "--part2");

    let mut input = String::new();
    print!("> ");
    io::stdout().flush().unwrap();
    while let Ok(_) = io::stdin().read_line(&mut input) {
        {
            let bytes = input.trim_right().as_bytes();
            if part2 {
                println!("{}", check_halfway_around(bytes).unwrap());
            } else {
                println!("{}", check_consecutive(bytes).unwrap());
            }
        }
        input.clear();
        print!("> ");
        io::stdout().flush().unwrap();
    }
}
