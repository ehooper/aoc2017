extern crate util;

enum State {
    A,
    B,
    C,
    D,
    E,
    F,
}
use State::*;

fn main() {
    let run = |input : &str| {
        if let Ok(steps) = input.trim().parse() {
            let tape = machine(steps);
            println!("{}", tape.iter().cloned().filter(|&c| c == 1).count());
        } else {
            eprintln!("invalid input");
        }
    };
    util::run_lines(run);
}

macro_rules! define_machine (
    ($name:ident : $($state:pat => $val0:expr, $mov0:expr, $state0:expr ; $val1:expr, $mov1:expr, $state1:expr)*) => {
        fn $name(steps : usize) -> Vec<u8> {
            let mut tape = vec![0; 1024];
            let mut state = A;
            let mut cursor : isize = 0;

            for _ in 0..steps {
                if cursor < 0 {
                    let len = tape.len();
                    tape.resize(len * 2, 0);
                    for i in 0..len {
                        tape.swap(i, i + len);
                    }
                    cursor += len as isize;
                } else if cursor >= tape.len() as isize {
                    let len = tape.len();
                    tape.resize(len * 2, 0);
                }
                #[allow(unreachable_patterns)]
                match state {
                    $($state => 
                      if tape[cursor as usize] == 0 {
                          tape[cursor as usize] = $val0;
                          cursor += $mov0;
                          state = $state0;
                      } else {
                          tape[cursor as usize] = $val1;
                          cursor += $mov1;
                          state = $state1;
                      }
                     ),*
                    _ => { }
                }
            }
            tape
        }
    };
);

define_machine!(machine :
                A => 1,  1, B ; 0, -1, B
                B => 1, -1, C ; 0,  1, E
                C => 1,  1, E ; 0, -1, D
                D => 1, -1, A ; 1, -1, A
                E => 0,  1, A ; 0,  1, F
                F => 1,  1, E ; 1,  1, A
               );

#[cfg(test)]
define_machine!(test_machine :
                A => 1,  1, B ; 0, -1, B
                B => 1, -1, A ; 1,  1, A
                );

#[test]
fn test_example() {
    assert_eq!(3, test_machine(6).iter().cloned().filter(|&c| c == 1).count());
}
