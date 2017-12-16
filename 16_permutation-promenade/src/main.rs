#[macro_use] extern crate nom;
extern crate util;

named!(num<&str, usize>, map_res!(nom::digit, str::parse));
named!(spin<&str, DanceMove>, do_parse!(
        char!('s') >>
        s: num     >>
        (Spin(s))
        ));
named!(exchange<&str, DanceMove>, do_parse!(
        char!('x') >>
        a: num     >>
        char!('/') >>
        b: num     >>
        (Exchange(a, b))
        ));
named!(letter<&str, char>, verify!(nom::anychar, |c| c >= 'a' && c <= 'z'));
named!(partner<&str, DanceMove>, do_parse!(
        char!('p')    >>
        a: letter     >>
        char!('/')    >>
        b: letter     >>
        (Partner(a as u8, b as u8))
        ));
named!(parse_input<&str, Vec<DanceMove>>,
       separated_nonempty_list_complete!(char!(','), alt!(spin | exchange | partner))
       );

#[derive(Clone, Copy)]
pub enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(u8, u8),
}
use DanceMove::*;

mod dance_line {
    use DanceMove;
    use DanceMove::*;

    #[derive(Clone)]
    pub struct DanceLine {
        line : [u8; 16],
        size : usize,
    }

    const START : [u8; 16] = *b"abcdefghijklmnop";

    impl DanceLine {
        pub fn new(size : usize) -> DanceLine {
            assert!(size <= 16);
            DanceLine { line: START, size }
        }

        pub fn reset(&mut self) { self.line = START; }

        pub fn dance(&mut self, dance : &[DanceMove]) {
            for &mov in dance.iter() {
                match mov {
                    Spin(len) => {
                        let temp = self.line;
                        let pos = self.size - len;
                        self.line[..len].copy_from_slice(&temp[pos..self.size]);
                        self.line[len..self.size].copy_from_slice(&temp[..pos]);
                    },
                    Exchange(a, b) => self.line[..self.size].swap(a, b),
                    Partner(a, b) => {
                        let ia = self.line[..self.size].iter().position(|&c| c == a).unwrap();
                        let ib = self.line[..self.size].iter().position(|&c| c == b).unwrap();
                        self.line.swap(ia, ib)
                    }
                }
            }
        }

        pub fn as_str(&self) -> &str { unsafe { ::std::str::from_utf8_unchecked(&self.line[..self.size]) } }
    }

    impl PartialEq for DanceLine {
        fn eq(&self, other : &DanceLine) -> bool { self.line == other.line }
    }
}

use dance_line::DanceLine;

fn find_cycle_length(mut tort : DanceLine, moves : &[DanceMove]) -> usize {
    let mut hare = tort.clone();
    hare.dance(moves);
    let mut meet_at : usize = 1;
    let mut length : usize = 1;
    while tort != hare {
        if length == meet_at {
            tort.clone_from(&hare);
            meet_at *= 2;
            length = 0;
        }
        hare.dance(moves);
        length += 1;
    }
    length
}

fn main() {
    use std::io::Read;

    let run = |input : &str| {
        let moves = if let Ok(input) = parse_input(input).to_result() { input }
        else {
            eprintln!("invalid input");
            return;
        };
        let mut line = DanceLine::new(16);
        line.dance(&moves);
        println!("first dance:     {}", line.as_str());
        line.reset();
        let cycle_length = find_cycle_length(line.clone(), &moves);
        for _ in 0..(1_000_000_000 % cycle_length) {
            line.dance(&moves);
        }
        println!("billionth dance: {}", line.as_str());
        println!("cycle length:    {}", cycle_length);
    };

    let mut input = String::new();

    if ! util::is_tty() {
        if std::io::stdin().read_to_string(&mut input).is_ok() {
            run(&input);
        }
        return;
    }

    util::prompt();
    while std::io::stdin().read_line(&mut input).is_ok() {
        run(input.trim_right());
        input.clear();
        util::prompt();
    }
}

#[test]
fn test_part_1() {
    let mut line = DanceLine::new(5);
    line.dance(&[Spin(1)]);
    assert_eq!("eabcd", line.as_str());
    line.dance(&[Exchange(3, 4)]);
    assert_eq!("eabdc", line.as_str());
    line.dance(&[Partner(b'e', b'b')]);
    assert_eq!("baedc", line.as_str());
}

#[test]
fn test_part_2() {
    let moves = parse_input("s1,x3/4,pe/b").to_result().unwrap();
    let mut line = DanceLine::new(5);
    line.dance(&moves);
    assert_eq!("baedc", line.as_str());
    line.dance(&moves);
    assert_eq!("ceadb", line.as_str());
}
