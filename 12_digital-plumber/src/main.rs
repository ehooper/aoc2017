#[macro_use] extern crate nom;

named!(pid<&str, u32>, verify!(map_res!(nom::digit, str::parse), |pid| pid <= 9999));
named!(parse_vertex<&str, (u32, Vec<u32>)>, ws!(tuple!(
            pid,
            preceded!(tag!("<->"), separated_nonempty_list_complete!(ws!(char!(',')), pid))
            )));

mod union_find {
    use std::collections::BTreeSet;

    #[derive(Debug)]
    pub struct UnionFind { root : Vec<u32>, rank : Vec<u32>, flattened : bool }

    impl UnionFind {
        pub fn new() -> UnionFind {
            UnionFind { root: Vec::new(), rank: Vec::new(), flattened: true }
        }

        pub fn extend(&mut self, extent : u32) {
            let start = self.root.len() as u32;
            if start <=  extent {
                self.root.extend(start..(extent + 1));
                self.rank.resize(self.root.len(), 0);
            }
        }

        pub fn find(&mut self, mut x : u32) -> u32 {
            while x != self.root[x as usize] {
                let root = self.root[x as usize];
                self.root[x as usize] = self.root[root as usize];
                x = self.root[x as usize];
            }
            x
        }

        pub fn union(&mut self, a : u32, b : u32) {
            let root_a = self.find(a);
            let root_b = self.find(b);

            if root_a == root_b {
                return;
            }

            let rank_a = self.rank[root_a as usize];
            let rank_b = self.rank[root_b as usize];

            if rank_a < rank_b {
                self.root[root_a as usize] = root_b;
            } else if rank_a > rank_b {
                self.root[root_b as usize] = root_a;
            } else {
                self.root[root_b as usize] = root_a;
                self.rank[root_a as usize] += 1;
            }
            self.flattened = false;
        }

        fn flatten(&mut self) {
            if ! self.flattened {
                for i in 0..(self.root.len() as u32) {
                    self.find(i);
                }
                self.flattened = true;
            }
        }

        /// Returns the size of the set containing `root`.
        pub fn set_size(&mut self, mut root : u32) -> usize {
            self.flatten();
            root = self.root.get(root as usize).map_or(0, |&r| r);
            self.root.iter().cloned().filter(|&r| r == root).count()
        }

        /// Returns the set of roots for each set in the forest.
        pub fn sets(&mut self) -> BTreeSet<u32> {
            self.flatten();
            self.root.iter().cloned().collect()
        }
    }
}

use union_find::UnionFind;

fn parse_input(input : &str) -> Result<UnionFind, nom::ErrorKind> {
    use nom::GetInput;
    let mut uf = UnionFind::new();
    for v in input.lines().map(parse_vertex) {
        if let Some(rem) = v.remaining_input() {
            if ! rem.is_empty() {
                eprintln!("warning: unconsumed: '{}'", rem);
            }
        }
        let (pid, siblings) = v.to_result()?;
        for sib in siblings {
            let extent = u32::max(pid, sib);
            uf.extend(extent);
            uf.union(pid, sib);
        }
    }
    Ok(uf)
}

fn prompt() {
    println!("Enter graph:");
}

fn main() {
    let run = |input : &str| {
        let parsed_input = parse_input(input);
        if let Ok(mut uf) = parsed_input {
            println!("size of group 0:  {}", uf.set_size(0));
            println!("number of groups: {}", uf.sets().len());
        } else {
            eprintln!("parse error: {:?}", parsed_input.unwrap_err());
        }
    };

    let mut input = String::new();
    let mut buffer = String::new();

    prompt();
    while let Ok(_) = std::io::stdin().read_line(&mut input) {
        if input.trim().is_empty() {
            run(&buffer);
            buffer.clear();
            println!();
            prompt();
        } else {
            buffer.push_str(input.as_ref());
        }
        input.clear();
    }
}

#[test]
fn test_example_input() {
    let input =
"0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";
    let mut uf = parse_input(input).expect("invalid input");
    assert_eq!(6, uf.set_size(0));
    assert_eq!(2, uf.sets().len());
}
