#[macro_use] extern crate nom;
extern crate libc;

named!(num<&str, usize>, map_res!(nom::digit, str::parse));
named!(range<&str, usize>, verify!(num, |r| r > 0));
named!(parse_layer<&str, (usize, usize)>, ws!(separated_pair!(num, char!(':'), range)));

fn severity(firewall : &[(usize, usize)], delay : usize) -> usize {
    let mut severity = 0;
    for &(depth, period) in firewall.iter() {
        if (depth + delay) % period == 0 {
            severity += depth * (period / 2 + 1);
        }
    }
    severity
}

fn min_delay(firewall : &[(usize, usize)]) -> usize {
    let firewall : Vec<(usize, usize)> = firewall.iter().cloned()
        .map(|(depth, period)| (period, (period - depth % period) % period))
        .collect();
    let mut delay = 0;
    while firewall.iter().cloned().any(|(period, offset)| delay % period == offset) {
        delay += 1;
    }
    delay
}

fn parse_input(input : &str) -> Option<Vec<(usize, usize)>> {
    let mut firewall = Vec::new();
    for line in input.lines().filter(|s| ! s.trim().is_empty()) {
        if let Ok((depth, range)) = parse_layer(line).to_result() {
            if range > 1 {
                let period = 2 * (range - 1);
                firewall.push((depth, period));
            } else if range == 1 {
                eprintln!("no possible solution for range 1");
                return None;
            }
        } else {
            eprintln!("unable to parse line: '{}'", line.trim());
            return None;
        }
    }
    Some(firewall)
}

fn prompt() {
    println!("Enter scanners:")
}

fn is_tty() -> bool {
    unsafe { libc::isatty(libc::STDIN_FILENO) != 0 }
}

fn main() {
    use std::io::Read;

    let run = |input : &str| {
        if let Some(input) = parse_input(input) {
            println!("severity(0):   {}", severity(&input, 0));
            println!("minimum delay: {}", min_delay(&input));
        }
    };

    let mut input = String::new();

    if ! is_tty() {
        std::io::stdin().read_to_string(&mut input).expect("invalid input");
        run(&input);
        return;
    }

    let mut buffer = String::new();

    prompt();
    while let Ok(_) = std::io::stdin().read_line(&mut buffer) {
        if buffer.trim().is_empty() {
            run(&input);
            input.clear();
            println!();
            prompt();
        } else {
            input.push_str(buffer.as_ref());
        }
        buffer.clear();
    }
}

#[test]
fn test_example() {
    let input = parse_input(
"0: 3
1: 2
4: 4
6: 4").unwrap();
    assert_eq!(24, severity(&input, 0));
    assert_eq!(10, min_delay(&input));
}
