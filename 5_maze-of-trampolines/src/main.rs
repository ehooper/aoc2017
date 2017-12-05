use std::io;

fn count_steps(jump_list : &mut [i32]) -> usize {
    let mut index : i32 = 0;
    let mut count : usize = 0;
    let len = jump_list.len() as i32;
    while index >= 0 && index < len {
        let temp = index as usize;
        index += jump_list[temp];
        jump_list[temp] += 1;
        count += 1;
    }
    count
}

fn count_steps_limit(jump_list : &mut [i32]) -> usize {
    let mut index : i32 = 0;
    let mut count : usize = 0;
    let len = jump_list.len() as i32;
    while index >= 0 && index < len {
        let temp = index as usize;
        index += jump_list[temp];
        if jump_list[temp] < 3 {
            jump_list[temp] += 1;
        } else {
            jump_list[temp] -= 1;
        }
        count += 1;
    }
    count
}

fn main() {
    let arg = std::env::args().nth(1);
    let part2 = arg.map_or(false, |s| s == "--part2");

    let mut jump_list = Vec::<i32>::new();
    let mut input = String::new();
    println!("Enter jump list:");
    while let Ok(_) = io::stdin().read_line(&mut input) {
        if let Ok(x) = input.trim_right().parse::<i32>() {
            jump_list.push(x);
        } else {
            let steps = if part2 { count_steps_limit(&mut jump_list) } else { count_steps(&mut jump_list) };
            println!("{} steps", steps);
            jump_list.clear();
            println!("Enter jump list:");
        }
        input.clear();
    }
}
