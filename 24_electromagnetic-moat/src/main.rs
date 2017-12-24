extern crate util;

type Component = (u32, u32);

fn add_strength(components : &[Component], mut used : &mut [bool], socket : u32, strength : u32) -> u32 {
    let mut strongest = strength;
    for (n, (i, o)) in components.iter().cloned().enumerate().filter(|&(_, (i, o))| i == socket || o == socket) {
        if ! used[n] {
            used[n] = true;
            let new_socket = if i == socket { o } else { i };
            let s = add_strength(components, &mut used, new_socket, i + o + strength);
            strongest = strongest.max(s);
            used[n] = false;
        }
    }
    strongest
}

fn add_length(components : &[Component], mut used : &mut [bool], socket : u32, strength : u32, length : u32) -> (u32, u32) {
    let mut longest = length;
    let mut strongest = strength;
    for (n, (i, o)) in components.iter().cloned().enumerate().filter(|&(_, (i, o))| i == socket || o == socket) {
        if ! used[n] {
            used[n] = true;
            let new_socket = if i == socket { o } else { i };
            let (s, l) = add_length(components, &mut used, new_socket, i + o + strength, 1 + length);
            if l > longest {
                longest = l;
                strongest = s;
            } else if l == longest {
                strongest = strongest.max(s);
            }
            used[n] = false;
        }
    }
    (strongest, longest)
}

fn get_strongest(input : &[(u32, u32)]) -> u32 {
    let mut used = vec![false; input.len()];
    add_strength(input, &mut used, 0, 0)
}

fn get_longest(input : &[(u32, u32)]) -> (u32, u32) {
    let mut used = vec![false; input.len()];
    add_length(input, &mut used, 0, 0, 0)
}

fn parse_input(input : &str) -> Result<Vec<Component>, String> {
    let mut components = Vec::new();
    for line in input.lines() {
        let sep = match line.find('/') {
            Some(i) => i,
            None => return Err(format!("no separator found in '{}'", line))
        };
        let (n1, n2) = line.split_at(sep);
        let s1 : u32 = match n1.trim().parse() {
            Ok(n) => n,
            _ => return Err(format!("could not parse number from '{}'", n1))
        };
        let s2 : u32 = match n2[1..].trim().parse() {
            Ok(n) => n,
            _ => return Err(format!("could not parse number from '{}'", n2))
        };
        components.push((s1, s2))
    }
    Ok(components)
}

fn main() {
    let run = |input : &str| {
        let components = match parse_input(input.trim()) {
            Err(s) => { eprintln!("{}", s); return; },
            Ok(c) => c
        };
        println!("strongest bridge: {}", get_strongest(&components));
        println!("longest bridge strength: {}", get_longest(&components).0);
    };
    util::run_multiline("enter components", run);
}

#[test]
fn test_example() {
    let input =
"0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";
    let components = parse_input(input).unwrap();
    assert_eq!(31, get_strongest(&components));
    assert_eq!((19, 4), get_longest(&components));
}
