extern crate util;
#[macro_use] extern crate nom;
extern crate clap;

pub type Register = char;

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Reg(Register),
    Num(i64)
}
use Value::*;

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Set(Register, Value),
    Sub(Register, Value),
    Mul(Register, Value),
    Jnz(Value, Value),
}
use Instruction::*;

mod parse {
    use super::{Register, Value, Instruction};
    use Value::*;
    use Instruction::*;
    use nom;

    named!(num<&str, i64>, map_res!(recognize!(preceded!(opt!(char!('-')), nom::digit)), str::parse));
    named!(register<&str, Register>, verify!(nom::anychar, |c| c >= 'a' && c <= 'h'));
    named!(value<&str, Value>, alt!(map!(num, Num) | map!(register, Reg)));

    macro_rules! reg_op {
        ($name : ident, $tag : expr, $variant : tt) => {
            named!($name<&str, Instruction>, ws!(do_parse!(
                        tag!($tag)  >>
                        r: register >>
                        v: value    >>
                        ($variant(r, v))
                        )));
        }
    }
    reg_op!(iset, "set", Set);
    reg_op!(isub, "sub", Sub);
    reg_op!(imul, "mul", Mul);

    named!(ijnz<&str, Instruction>, ws!(do_parse!(
                tag!("jnz") >>
                v1: value   >>
                v2: value   >>
                (Jnz(v1, v2))
                )));

    named!(pub parse_program<&str, Vec<Instruction>>,
           complete!(many0!(alt!(iset | isub | imul | ijnz)))
          );
}

use parse::parse_program;

mod process {
    use super::{Register, Value, Instruction};
    use Value::*;
    use Instruction::*;

    struct Processor {
        registers : [i64; 8]
    }

    macro_rules! bin_op {
        ($name : ident, $op : tt) => {
            fn $name(&mut self, r : Register, v : Value) {
                let a = self.get(Reg(r));
                let b = self.get(v);
                self.set(r, Num(a $op b));
            }
        }
    }

    impl Processor {
        fn new() -> Processor { Processor { registers: [0; 8] } }

        fn get(&self, val : Value) -> i64 {
            match val {
                Num(v) => v,
                Reg(r) => self.registers[(r as u8 - b'a') as usize]
            }
        }

        fn set(&mut self, reg : Register, val : Value) {
            self.registers[(reg as u8 - b'a') as usize] = self.get(val);
        }

        bin_op!(sub, -);
        bin_op!(mul, *);
    }

    pub enum ProcessState {
        Ran(Instruction),
        Halt(i64),
    }
    use ProcessState::*;

    pub struct Process<'a> {
        processor : Processor,
        ip : usize,
        program : &'a [Instruction],
    }

    impl<'a> Process<'a> {
        pub fn new(flag : i64, program : &'a [Instruction]) -> Process<'a> {
            let mut processor = Processor::new();
            processor.set('a', Num(flag));
            Process { processor, ip: 0, program }
        }

        pub fn step(&mut self) -> ProcessState {
            if self.ip >= self.program.len() {
                return Halt(self.ip as i64);
            }
            let instruction = self.program[self.ip];
            match instruction {
                Set(x, y) => self.processor.set(x, y),
                Sub(x, y) => self.processor.sub(x, y),
                Mul(x, y) => self.processor.mul(x, y),
                Jnz(x, y) => {
                    let x = self.processor.get(x);
                    if x != 0 {
                        let y = self.processor.get(y);
                        self.ip = (self.ip as i64 + y) as usize;
                        return Ran(instruction);
                    }
                }
            }
            self.ip += 1;
            Ran(instruction)
        }

        pub fn get_register(&self, reg : Register) -> i64 {
            self.processor.get(Reg(reg))
        }
    }
}

use process::{Process, ProcessState};
use ProcessState::*;

fn run_mul_count(program : &[Instruction]) -> usize {
    let mut mul_count = 0;
    let mut process = Process::new(0, program);
    loop {
        match process.step() {
            Ran(Mul(_, _)) => { mul_count += 1 },
            Halt(_) => {
                return mul_count;
            },
            _ => { }
        }
    }
}

fn print_program(program : &[Instruction]) {
    let print_val = |v : Value| -> String {
        match v {
            Reg(r) => format!("{}", r as char),
            Num(n) => format!("{}", n)
        }
    };
    for (line, &i) in program.iter().enumerate() {
        print!("{:2}: ", line + 1);
        match i {
            Set(r, v) => println!("{} = {};", r as char, print_val(v)),
            Sub(r, v) => println!("{} -= {};", r as char, print_val(v)),
            Mul(r, v) => println!("{} *= {};", r as char, print_val(v)),
            Jnz(a, Num(o)) => println!("if ({} != 0)\n\tgoto {};", print_val(a), (line as i64 + o + 1)),
            _ => eprintln!("unexpected instruction on line {}", line)
        }
    }
}

fn bruteforce(program : &[Instruction]) -> i64 {
    let mut process = Process::new(1, program);
    loop {
        if let Halt(_) = process.step() {
            return process.get_register('h');
        }
    }
}

fn main() {
    use clap::{App, Arg};
    let options = App::new("Day 23: Coprocessor Conflation")
        .about("\nSolve part one (by default), or part two either by hand or by brute force. See README.md for solving part two by hand.")
        .arg(Arg::with_name("part1")
             .long("part1")
             .help("Solves part one (default)")
             .conflicts_with("part2")
             )
        .arg(Arg::with_name("part2")
             .long("part2")
             .value_name("METHOD")
             .possible_values(&["label", "brute-force"])
             .default_value("label")
            )
        .after_help("For solving part two, either manually decompile the program from the output of '--part2 label' \
                    (see README.md) or brute force the solution with '--part2 brute-force' \
                    (will take a very long time)")
        .get_matches();

    let (label, brute) = if options.occurrences_of("part2") == 0 {
        (false, false)
    } else {
        match options.value_of("part2") {
            Some("label") => (true, false),
            Some("brute-force") => (false, true),
            _ => (false, false)
        }
    };

    let run = |input : &str| {
        let program = match parse_program(input).to_result() {
            Ok(parsed) => parsed,
            Err(_) => {
                eprintln!("invalid input");
                return;
            },
        };
        if label {
            print_program(&program);
        } else if brute {
            println!("value of h: {}", bruteforce(&program));
        } else  {
            println!("debug multiplications: {}", run_mul_count(&program));
        }
    };
    util::run_multiline("enter program:", run);
}
