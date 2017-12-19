extern crate util;
#[macro_use] extern crate nom;

pub type Register = char;

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Reg(Register),
    Num(i64)
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Snd(Value),
    Set(Register, Value),
    Add(Register, Value),
    Mul(Register, Value),
    Mod(Register, Value),
    Rcv(Register),
    Jgz(Value, Value),
}

mod parse {
    use super::{Register, Value, Instruction};
    use Value::*;
    use Instruction::*;
    use nom;

    named!(num<&str, i64>, map_res!(recognize!(preceded!(opt!(char!('-')), nom::digit)), str::parse));
    named!(register<&str, Register>, verify!(nom::anychar, |c| c >= 'a' && c <= 'z'));
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
    reg_op!(iadd, "add", Add);
    reg_op!(imul, "mul", Mul);
    reg_op!(imod, "mod", Mod);

    named!(isnd<&str, Instruction>, ws!(do_parse!(
                tag!("snd") >>
                v: value    >>
                (Snd(v))
                )));
    named!(ircv<&str, Instruction>, ws!(do_parse!(
                tag!("rcv") >>
                r: register >>
                (Rcv(r))
                )));
    named!(ijgz<&str, Instruction>, ws!(do_parse!(
                tag!("jgz") >>
                v1: value   >>
                v2: value   >>
                (Jgz(v1, v2))
                )));
    named!(pub parse_program<&str, Vec<Instruction>>,
           complete!(many0!(alt!(isnd | iset | iadd | imul | imod | ircv | ijgz)))
          );
}

use parse::parse_program;

mod process {
    use super::{Register, Value, Instruction};
    use Value::*;
    use Instruction::*;

    struct Processor {
        registers : [i64; 26]
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
        fn new() -> Processor { Processor { registers: [0; 26] } }

        fn get(&self, val : Value) -> i64 {
            match val {
                Num(v) => v,
                Reg(r) => self.registers[(r as u8 - b'a') as usize]
            }
        }

        fn set(&mut self, reg : Register, val : Value) {
            self.registers[(reg as u8 - b'a') as usize] = self.get(val);
        }

        bin_op!(add, +);
        bin_op!(mul, *);
        bin_op!(modulus, %);
    }

    pub enum ProcessState {
        Running,
        Waiting(Register),
        Sending(i64),
        Invalid(i64),
    }
    use ProcessState::*;

    pub struct Process<'a> {
        processor : Processor,
        ip : usize,
        program : &'a [Instruction],
    }

    impl<'a> Process<'a> {
        pub fn new(pid : i64, program : &'a [Instruction]) -> Process<'a> {
            let mut processor = Processor::new();
            processor.set('p', Num(pid));
            Process { processor, ip: 0, program }
        }

        pub fn step(&mut self) -> ProcessState {
            if self.ip >= self.program.len() {
                return Invalid(self.ip as i64);
            }
            match self.program[self.ip] {
                Snd(x) => {
                    self.ip += 1;
                    return Sending(self.processor.get(x))
                },
                Set(x, y) => self.processor.set(x, y),
                Add(x, y) => self.processor.add(x, y),
                Mul(x, y) => self.processor.mul(x, y),
                Mod(x, y) => self.processor.modulus(x, y),
                Rcv(r) => return Waiting(r),
                Jgz(x, y) => {
                    let x = self.processor.get(x);
                    if x > 0 {
                        let y = self.processor.get(y);
                        self.ip = (self.ip as i64 + y) as usize;
                        return Running;
                    }
                }
            }
            self.ip += 1;
            Running
        }

        pub fn get_register(&self, reg : Register) -> i64 {
            self.processor.get(Reg(reg))
        }

        pub fn receive(&mut self, reg : Register, val : i64) {
            self.processor.set(reg, Num(val));
            self.ip += 1;
        }
    }
}

use process::{Process, ProcessState};
use ProcessState::*;

fn run_solo(program : &[Instruction]) -> Result<i64, String> {
    let mut process = Process::new(0, program);
    let mut freq = 0;
    loop {
        match process.step() {
            Running => {},
            Sending(f) => freq = f,
            Waiting(r) => {
                let x = process.get_register(r);
                if x != 0 {
                    return Ok(freq);
                }
                process.receive(r, x);
            },
            Invalid(ip) => return Err(format!("invalid instruction: {}", ip)),
        }
    }
}

#[allow(dead_code)]
fn run_duet_single(program : &[Instruction]) -> Result<usize, String> {
    use std::collections::vec_deque::VecDeque;

    let mut sends : usize = 0;
    let mut p0 = Process::new(0, program);
    let mut p1 = Process::new(1, program);
    let mut q0 = VecDeque::new();
    let mut q1 = VecDeque::new();
    let mut p0_waiting = false;
    let mut p1_waiting = false;
    loop {
        match p0.step() {
            Running => {},
            Sending(x) => q1.push_back(x),
            Waiting(r) => if let Some(x) = q0.pop_front() {
                p0.receive(r, x);
                p0_waiting = false;
            } else { p0_waiting = true },
            Invalid(ip) => return Err(format!("invalid instruction for process 0: {}", ip)),
        }
        match p1.step() {
            Running => {},
            Sending(x) => { sends += 1; q0.push_back(x) },
            Waiting(r) => if let Some(x) = q1.pop_front() {
                p1.receive(r, x);
                p1_waiting = false;
            } else { p1_waiting = true },
            Invalid(ip) => return Err(format!("invalid instruction for process 1: {}", ip)),
        }
        if p0_waiting && p1_waiting && q0.is_empty() && q1.is_empty() {
            return Ok(sends);
        }
    }
}

use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{Arc, Mutex};

/// indexed by pid, true if running (i.e., not waiting or terminated).
type ProcessStates = Arc<Mutex<[bool; 2]>>;

fn program_thread(
    pid : i64,
    program : &Arc<Vec<Instruction>>,
    (send, recv) : (Sender<i64>, Receiver<i64>),
    state : &ProcessStates) -> Result<i64, String> {

    let mut process = Process::new(pid, program);
    let pid = pid as usize;
    let mut sent = 0;
    loop {
        match process.step() {
            Running => {},
            Sending(x) => {
                send.send(x).unwrap();
                sent += 1;
            },
            Waiting(r) => {
                loop {
                    if let Ok(x) = recv.try_recv() {
                        process.receive(r, x);
                        break;
                    }
                    let mut is_running = state.lock().unwrap();
                    is_running[pid] = false;
                    if ! is_running[(pid + 1) % 2] {
                        return Ok(sent);
                    }
                }
                state.lock().unwrap()[pid] = true;
            },
            Invalid(ip) => {
                state.lock().unwrap()[pid] = false;
                return Err(format!("invalid instruction for process {}: {}", pid, ip))
            },
        }
    }
}

fn run_duet_multi(program : Vec<Instruction>) -> Result<i64, String> {
    let program = Arc::new(program);
    let state = Arc::new(Mutex::new([true, true]));
    let (i0, o0) = channel();
    let (i1, o1) = channel();

    let s = Arc::clone(&state);
    let p = Arc::clone(&program);
    let _p0 = std::thread::spawn(move || program_thread(0, &p, (i0, o1), &s));

    let s = Arc::clone(&state);
    let p = Arc::clone(&program);
    let p1 = std::thread::spawn(move || program_thread(1, &p, (i1, o0), &s));

    p1.join().unwrap()
}

fn main() {
    let run = |input : &str| {
        let program = match parse_program(input).to_result() {
            Ok(parsed) => parsed,
            Err(_) => {
                eprintln!("invalid input");
                return;
            },
        };
        match run_solo(&program) {
            Ok(result) => println!("last frequency: {}", result),
            Err(msg) => eprintln!("{}", msg),
        }
        match run_duet_multi(program) {
            Ok(result) => println!("sends for p1:   {}", result),
            Err(msg) => eprintln!("{}", msg),
        }
    };
    util::run_multiline("enter program:", run);
}

#[test]
fn test_part_1() {
    let input =
"set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";
    let program = parse_program(input).to_result().unwrap();
    assert_eq!(Ok(4), run_solo(&program));
}

#[test]
fn test_part_2() {
    let input =
"snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d";
    let program = parse_program(input).to_result().unwrap();
    assert_eq!(Ok(3), run_duet_single(&program));
    assert_eq!(Ok(3), run_duet_multi(program));
}
