#[derive(Debug, Clone, Copy)]
enum HexDir {
    N,
    NE,
    SE,
    S,
    SW,
    NW,
}

use HexDir::*;

impl HexDir {
    fn apply_step(self, (x, y) : (i32, i32)) -> (i32, i32) {
        match self {
            N  => (x    , y + 2),
            NE => (x + 1, y + 1),
            SE => (x + 1, y - 1),
            S  => (x    , y - 2),
            SW => (x - 1, y - 1),
            NW => (x - 1, y + 1),
        }
    }
}

use std::str::FromStr;

impl FromStr for HexDir {
    type Err = String;

    fn from_str(s : &str) -> Result<Self, Self::Err> {
        use std::ascii::AsciiExt;

        let mut si = *b"  ";
        let len = s.len().min(2);
        si[..len].copy_from_slice(&s.as_bytes()[..len]);
        si.make_ascii_lowercase();
        match &si {
            b"n " => Ok(N),
            b"ne" => Ok(NE),
            b"se" => Ok(SE),
            b"s " => Ok(S),
            b"sw" => Ok(SW),
            b"nw" => Ok(NW),
            _ => Err(format!("could not parse direction: '{}'", s)),
        }
    }
}

fn distance((x, y) : (i32, i32)) -> i32 {
    let xd = x.abs();
    let yd = y.abs();
    xd + i32::max(0, (yd - xd) / 2)
}

struct PathDistance {
    steps : i32,
    max : i32,
}

fn path_distance(path : &[HexDir]) -> PathDistance {
    let mut max = 0;
    let d = path.iter().fold((0, 0), |mut acc, hd| {
        acc = hd.apply_step(acc);
        max = i32::max(max, distance(acc));
        acc
    });
    PathDistance { steps: distance(d), max }
}

fn parse_input(input : &str) -> Result<Vec<HexDir>, String> {
    input.split(',').map(str::trim).filter(|s| ! s.is_empty()).map(str::parse).collect()
}

fn prompt() {
    use std::io::Write;

    print!("> ");
    std::io::stdout().flush().unwrap();
}

fn main() {
    use std::io::prelude::*;

    let run = |input : &str| {
        let parsed_input = parse_input(input);
        if let Ok(input) = parsed_input {
            let answer = path_distance(&input);
            println!("steps: {}", answer.steps);
            println!("max: {}", answer.max);
        } else {
            eprintln!("{}", parsed_input.unwrap_err());
        }
    };

    let mut input = String::new();

    if let Some(filename) = std::env::args_os().nth(1) {
        let mut file = std::fs::File::open(filename).expect("file not found");
        file.read_to_string(&mut input).expect("error reading file");
        run(&input);
        return;
    }

    prompt();
    while let Ok(_) = std::io::stdin().read_line(&mut input) {
        run(&input);
        input.clear();
        prompt();
    }
}

#[test]
fn test_part_1() {
    assert_eq!(3, path_distance(&[NE, NE, NE]).steps);
    assert_eq!(0, path_distance(&[NE, NE, SW, SW]).steps);
    assert_eq!(2, path_distance(&[NE, NE, S, S]).steps);
    assert_eq!(3, path_distance(&[SE, SW, SE, SW, SW]).steps);
}
