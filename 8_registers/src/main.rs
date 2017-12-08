#[macro_use] extern crate nom;
extern crate regex;

use std::io;
use std::collections::BTreeMap;
use std::str::FromStr;

named!(integer<&str, i32>, map_res!(re_find!("-?[0-9]+"), FromStr::from_str));
named!(register<&str, Register>, map_res!(re_find!("[a-zA-Z]{1,3}"), parse_register));
named!(parse_instruction<&str, (Register, Op, i32, Register, Cond, i32)>, ws!(tuple!(
            register,
            map_res!(nom::alpha, FromStr::from_str),
            integer,
            preceded!(tag_no_case!("if"), register),
            // NB: parsers are applied from left to right, so "<=" MUST come before "<" to parse correctly!
            // Got the wrong answer because of this.
            map_res!(alt!(tag!("<=") | tag!("<") | tag!("==") | tag!("!=") | tag!(">=") | tag!(">")), FromStr::from_str),
            integer
            )));

#[derive(Debug, PartialEq, Eq)]
enum Cond {
    LT,
    LE,
    EQ,
    NE,
    GE,
    GT,
}

impl FromStr for Cond {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "<" => Ok(Cond::LT),
            "<=" => Ok(Cond::LE),
            "==" => Ok(Cond::EQ),
            "!=" => Ok(Cond::NE),
            ">=" => Ok(Cond::GE),
            ">" => Ok(Cond::GT),
            _ => Err("invalid conditional operator"),
        }
    }
}

impl Cond {
    fn check(&self, a : i32, b : i32) -> bool {
        use Cond::*;
        match *self {
            LT => a < b,
            LE => a <= b,
            EQ => a == b,
            NE => a != b,
            GE => a >= b,
            GT => a > b,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Op {
    Inc,
    Dec,
}

use Op::*;

impl FromStr for Op {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_ref() {
            "INC" => Ok(Inc),
            "DEC" => Ok(Dec),
            _ => Err("invalid operation"),
        }
    }
}

type Register = [u8; 3];

fn parse_register(s : &str) -> Result<Register, &str> {
    let mut r : Register = [0; 3];
    let len = s.len();
    if len > 3 {
        return Err("register name too long (3 byte max)");
    }
    r[..len].copy_from_slice(s.as_bytes());
    Ok(r)
}

fn main() {
    let mut input = String::new();

    let mut registers = BTreeMap::<Register, i32>::new();
    let mut max_held = 0;
    println!("Enter program:");
    while let Ok(_) = io::stdin().read_line(&mut input) {
        {
            let input = input.trim_right();
            if input.is_empty() {
                let max = *registers.values().max().unwrap_or(&0);
                println!("max register value: {}", max);
                println!("max held: {}", max_held);
                max_held = 0;
                registers.clear();
                println!("Enter program:");
            } else if let Ok((r1, op, amount, r2, cond, value))
                = parse_instruction(input).to_result() {
                    let r2 = *registers.entry(r2).or_insert(0);
                    let r1 = registers.entry(r1).or_insert(0);
                    if cond.check(r2, value) {
                        *r1 += match op {
                            Inc => amount,
                            Dec => -amount,
                        };
                        max_held = i32::max(max_held, *r1);
                    }
            } else {
                eprintln!("Invalid instruction: '{}'", input);
            }
        }
        input.clear();
    }
}

#[test]
fn test_integer_parser() {
    use nom::*;
    let empty = "";
    assert_eq!(integer("0"), IResult::Done(empty, 0));
    assert_eq!(integer("5"), IResult::Done(empty, 5));
    assert_eq!(integer("1234"), IResult::Done(empty, 1234));
    assert_eq!(integer("-1234"), IResult::Done(empty, -1234));
}

#[test]
fn test_register_parser() {
    use nom::*;
    let empty = "";
    assert_eq!(register("a"), IResult::Done(empty, *b"a\0\0"));
    assert_eq!(register("ab"), IResult::Done(empty, *b"ab\0"));
    assert_eq!(register("abc"), IResult::Done(empty, *b"abc"));
}

#[test]
fn test_instruction_parser() {
    use nom::*;
    let empty = "";
    assert_eq!(parse_instruction("a inc 5 if b < 10"), IResult::Done(empty, (*b"a\0\0", Op::Inc, 5, *b"b\0\0", Cond::LT, 10)));
    assert_eq!(parse_instruction("a inc 5 if b <= 10"), IResult::Done(empty, (*b"a\0\0", Op::Inc, 5, *b"b\0\0", Cond::LE, 10)));
    assert_eq!(parse_instruction("abc DEC 5 IF xyz != -30"), IResult::Done(empty, (*b"abc", Op::Dec, 5, *b"xyz", Cond::NE, -30)));
}
