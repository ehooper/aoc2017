use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter spreadsheet:");
    let mut checksum : i32 = 0;
    let mut in_sheet = false;
    while let Ok(_) = io::stdin().read_line(&mut input) {
        let mut max = std::i32::MIN;
        let mut min = std::i32::MAX;
        let mut size : u32 = 0;
        {
            let iter = input.trim_right().split_whitespace().map(|s| s.parse::<i32>().unwrap());
            for i in iter {
                size += 1 ;
                if i > max {
                    max = i;
                }
                if i < min {
                    min = i;
                }
            }
        }
        if in_sheet && size == 0 {
            println!("Checksum: {}", checksum);
            println!("Enter spreadsheet:");
            in_sheet = false;
            checksum = 0;
        }
        if size > 0 {
            in_sheet = true;
            checksum += max - min;
        }
        input.clear();
    }
}
