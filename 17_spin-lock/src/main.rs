extern crate util;

fn main() {
    let run = |input : &str| {
        let steps = match str::parse(input.trim()) {
            Ok(n) => n,
            Err(_) => {
                eprintln!("invalid input");
                return;
            }
        };
        println!("part one: {:?}", simulate(steps));
        println!("part two: {:?}", simulate_after_zero(steps));
    };

    util::run_lines(run);
}

fn simulate(step : usize) -> u32 {
    let mut buffer : Vec<u32> = Vec::with_capacity(2018);
    buffer.push(0);
    let mut pos = 0;
    for i in 1..2018 {
        pos = (pos + step + 1) % i;
        buffer.insert(pos + 1, i as u32);
    }
    let pos = buffer.iter().position(|&n| n == 2017).unwrap();
    if pos < 2017 {
        buffer[pos + 1]
    } else {
        buffer[0]
    }
}

fn simulate_after_zero(step : usize) -> u32 {
    let mut pos = 0;
    let mut val = 0;
    for i in 1..50_000_001 {
        pos = (pos + step + 1) % i;
        if pos == 0 {
            val = i as u32;
        }
    }
    val
}

#[test]
fn test_part_1() {
    assert_eq!(638, simulate(3));
}
