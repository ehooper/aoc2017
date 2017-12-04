use std::io;
use std::io::Write;

fn main() {
    let mut input = String::new();
    print!("> ");
    io::stdout().flush().unwrap();
    while let Ok(_) = io::stdin().read_line(&mut input) {
        let mut sum : isize = 0;
        {
            let bytes = input.trim_right().as_bytes();
            let iter = bytes.iter().zip(bytes.iter().cycle().skip(1));
            for (&d1, &d2) in iter {
                if d1 < b'0' || d1 > b'9' {
                    println!("error: invalid digit '{}'", d1);
                    break;
                }
                if d1 == d2 {
                    sum += (d1 as isize) - (b'0' as isize);
                }
            }
        }
        input.clear();
        println!("{}", sum);
        print!("> ");
        io::stdout().flush().unwrap();
    }
}
