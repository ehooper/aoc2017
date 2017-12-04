use std::io;

fn checksum_diff(nums : &Vec<i32>) -> i32 {
    let mut max = std::i32::MIN;
    let mut min = std::i32::MAX;
    for &i in nums {
        if i > max {
            max = i;
        }
        if i < min {
            min = i;
        }
    }
    max - min
}

fn checksum_div(nums : &Vec<i32>) -> i32 {
    let mut iter = nums.iter();
    while let Some(x) = iter.next() {
        for y in iter.clone() {
            if x >= y && x % y == 0 {
                return x / y;
            } else if x < y && y % x == 0 {
                return y / x;
            }
        }
    }
    return 0;
}

fn main() {
    let arg = std::env::args().nth(1);
    let part2 = arg.map_or(false, |s| s == "--part2");

    let mut input = String::new();
    println!("Enter spreadsheet:");
    let mut checksum : i32 = 0;
    let mut in_sheet = false;
    while let Ok(_) = io::stdin().read_line(&mut input) {
        let nums : Vec<i32> = input.trim_right().split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
        let size = nums.len();
        if in_sheet && size == 0 {
            println!("Checksum: {}", checksum);
            println!("Enter spreadsheet:");
            in_sheet = false;
            checksum = 0;
        }
        if size > 0 {
            in_sheet = true;
            if part2 {
                checksum += checksum_div(&nums);
            } else {
                checksum += checksum_diff(&nums);
            }
        }
        input.clear();
    }
}
