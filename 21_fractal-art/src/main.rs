#[macro_use] extern crate nom;
extern crate util;

enum RuleType {
    Rule2(Rule<Pat2, Pat3>),
    Rule3(Rule<Pat3, Pat4>),
}
use RuleType::*;

macro_rules! parse_pat (
    ($name : ident, $len : expr) => (
        fn $name(input : &str) -> Result<[u8; $len * $len], ()> {
            let mut pat = [0; $len * $len];
            {
                let mut iter = pat.iter_mut();
                for c in input.chars().filter(|&c| c == '.' || c == '#') {
                    match iter.next() {
                        Some(v) => *v = c as u8,
                        None => return Err(())
                    }
                }
                if iter.next().is_some() {
                    return Err(());
                }
            }
            Ok(pat)
        }
    )
);

parse_pat!(parse_pat2, 2);
parse_pat!(parse_pat3, 3);
parse_pat!(parse_pat4, 4);

named!(rule2<&str, RuleType>, ws!(do_parse!(
        i: map_res!(is_a_s!("#./"), parse_pat2) >>
        tag!("=>") >>
        o: map_res!(is_a_s!("#./"), parse_pat3) >>
        (Rule2(Rule(patterns(&i), o)))
        )));
named!(rule3<&str, RuleType>, ws!(do_parse!(
        i: map_res!(is_a_s!("#./"), parse_pat3) >>
        tag!("=>") >>
        o: map_res!(is_a_s!("#./"), parse_pat4) >>
        (Rule3(Rule(patterns(&i), o)))
        )));
named!(parse_rule<&str, RuleType>, alt!(rule2 | rule3));

fn parse_input(input : &str) -> Result<(Vec<Rule<Pat2, Pat3>>, Vec<Rule<Pat3, Pat4>>), ()> {
    let mut rules2 = Vec::new();
    let mut rules3 = Vec::new();
    for line in input.lines() {
        match parse_rule(line).to_result().map_err(|_| ())? {
            Rule2(r) => rules2.push(r),
            Rule3(r) => rules3.push(r)
        }
    }
    Ok((rules2, rules3))
}

type Pat2 = [u8; 4];
type Pat3 = [u8; 9];
type Pat4 = [u8; 16];

trait PatternElem : Copy + Default + PartialEq + std::fmt::Debug { }
impl PatternElem for u8 { }

trait Pattern : Copy + Default + PartialEq + std::fmt::Debug { }
impl<E : PatternElem> Pattern for [E; 4] { }
impl<E : PatternElem> Pattern for [E; 9] { }
impl<E : PatternElem> Pattern for [E; 16] { }

trait SmallMatrix : Pattern {
    fn rotate(&self) -> Self;
    fn flip_v(&self) -> Self;
    fn flip_h(&self) -> Self;
}

type PatSym<P> = [P; 8];

#[derive(Debug)]
struct Rule<P1 : SmallMatrix, P2 : Pattern>(PatSym<P1>, P2);

impl<T : PatternElem> SmallMatrix for [T; 4] {
    fn rotate(&self) -> Self {
        [self[1], self[3], self[0], self[2]]
    }
    fn flip_v(&self) -> Self {
        [self[2], self[3], self[0], self[1]]
    }
    fn flip_h(&self) -> Self {
        [self[1], self[0], self[3], self[2]]
    }
}

impl<T : PatternElem> SmallMatrix for [T; 9] {
    fn rotate(&self) -> Self {
        [self[2], self[5], self[8], self[1], self[4], self[7], self[0], self[3], self[6]]
    }
    fn flip_v(&self) -> Self {
        [self[6], self[7], self[8], self[3], self[4], self[5], self[0], self[1], self[2]]
    }
    fn flip_h(&self) -> Self {
        [self[2], self[1], self[0], self[5], self[4], self[3], self[8], self[7], self[6]]
    }
}

fn patterns<M : SmallMatrix>(pat : &M) -> PatSym<M> {
    let mut output = [M::default(); 8];
    {
        let mut iter = output.iter_mut();
        let mut sym = *pat;
        for _ in 0..4 {
            *iter.next().unwrap() = sym;
            *iter.next().unwrap() = sym.flip_v();
            sym = sym.rotate();
        }
    }
    output
}

fn match_rule<I : SmallMatrix, O : Pattern>(pat : &I, rules : &[Rule<I, O>]) -> Option<O> {
    for &Rule(ref i, ref o) in rules {
        if i.iter().any(|r| pat == r) {
            return Some(*o);
        }
    }
    None
}

#[derive(Debug)]
enum ApplyError {
    InvalidSize,
    BadPattern2(Pat2),
    BadPattern3(Pat3),
}

fn apply_rules_2(input : &[u8], buffer : &mut Vec<u8>, size : usize, rules : &[Rule<Pat2, Pat3>]) -> Result<(), ApplyError> {
    let new_size = (size * 3) / 2;
    buffer.resize((input.len() * 9) / 4, 0);
    let len = size / 2;
    for i in 0..len {
        for j in 0..len {
            let pat = [input[i * 2 * size + j * 2], input[i * 2 * size + (j * 2 + 1)], input[(i * 2 + 1) * size + j * 2], input[(i * 2 + 1) * size + (j * 2 + 1)]];
            let output = match match_rule(&pat, rules) {
                Some(pattern) => pattern,
                None => return Err(ApplyError::BadPattern2(pat)),
            };
            for ii in 0..3 {
                for jj in 0..3 {
                    buffer[(i * 3 + ii) * new_size + j * 3 + jj] = output[ii * 3 + jj];
                }
            }
        }
    }
    Ok(())
}

fn apply_rules_3(input : &[u8], buffer : &mut Vec<u8>, size : usize, rules : &[Rule<Pat3, Pat4>]) -> Result<(), ApplyError> {
    let new_size = (size * 4) / 3;
    buffer.resize((input.len() * 16) / 9, 0);
    let len = size / 3;
    for i in 0..len {
        for j in 0..len {
            let mut pat = [0; 9];
            for ii in 0..3 {
                for jj in 0..3 {
                    pat[ii * 3 + jj] = input[(i * 3 + ii) * size + j * 3 + jj];
                }
            }
            let output = match match_rule(&pat, rules) {
                Some(pattern) => pattern,
                None => return Err(ApplyError::BadPattern3(pat)),
            };
            for ii in 0..4 {
                for jj in 0..4 {
                    buffer[(i * 4 + ii) * new_size + j * 4 + jj] = output[ii * 4 + jj];
                }
            }
        }
    }
    Ok(())
}

fn apply_rules(input : &mut Vec<u8>, buffer : &mut Vec<u8>, size : &mut usize, patterns2 : &[Rule<Pat2, Pat3>], patterns3 : &[Rule<Pat3, Pat4>]) -> Result<(), ApplyError> {
    let result;
    if *size % 2 == 0 {
        result = apply_rules_2(input, buffer, *size, patterns2);
        input.clone_from(buffer);
        *size = (*size * 3) / 2
    } else if *size % 3 == 0 {
        result = apply_rules_3(input, buffer, *size, patterns3);
        input.clone_from(buffer);
        *size = (*size * 4) / 3
    } else {
        return Err(ApplyError::InvalidSize)
    }
    result
}

fn print_pattern(pat : &[u8], size : usize) {
    for r in 0..size {
        println!("{}", std::str::from_utf8(&pat[(r * size)..((r + 1) * size)]).unwrap());
    }
}

fn main() {
    let run = |input : &str| {
        let arg : Option<usize> = std::env::args().nth(1).and_then(|s| s.parse().map(Some).unwrap_or(None));
        let mut image = b".#...####".to_vec();
        let mut buffer = Vec::new();
        let mut size = 3;
        let (rules2, rules3) = match parse_input(input) {
            Ok(rules) => rules,
            Err(_) => { eprintln!("invalid input"); return; }
        };
        if let Some(iterations) = arg {
            let iterations = if iterations <= 20 { iterations } else {
                eprintln!("large number of iterations, limiting to 20");
                20
            };
            for _ in 0..iterations {
                if let Err(e) = apply_rules(&mut image, &mut buffer, &mut size, &rules2, &rules3) {
                    eprintln!("{:?}", e);
                    return;
                }
            }
            print_pattern(&image, size);
            return;
        }
        for _ in 0..5 {
            if let Err(e) = apply_rules(&mut image, &mut buffer, &mut size, &rules2, &rules3) {
                eprintln!("{:?}", e);
                return;
            }
        }
        println!("Pixels on after 5 iterations: {}", image.iter().cloned().filter(|&c| c == b'#').count());
        for _ in 5..18 {
            if let Err(e) = apply_rules(&mut image, &mut buffer, &mut size, &rules2, &rules3) {
                eprintln!("{:?}", e);
                return;
            }
        }
        println!("Pixels on after 18 iterations {}", image.iter().cloned().filter(|&c| c == b'#').count());
    };
    util::run_multiline("enter image", run);
}

#[test]
fn test_example() {
    let input =
"../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#";
    let (rules2, rules3) = parse_input(input);
    let mut input = ".#...####".as_bytes().to_vec();
    let mut buffer = Vec::new();
    let mut size = 3;
    apply_rules(&mut input, &mut buffer, &mut size, &rules2, &rules3).unwrap();
    assert_eq!(b"#..#........#..#", image.as_ref());
    apply_rules(&mut input, &mut buffer, &mut size, &rules2, &rules3).unwrap();
    assert_eq!(b"##.##.#..#........##.##.#..#......", image.as_ref());
}
