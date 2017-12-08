#[macro_use] extern crate nom;
extern crate petgraph;

use std::io;
use std::collections::BTreeMap;
use std::str::FromStr;
use nom::alpha;
use petgraph::{Graph, Direction};
use petgraph::graph::NodeIndex;

named!(weight<&str, u32>, ws!(delimited!(tag!("("), map_res!(nom::digit, FromStr::from_str), tag!(")"))));
named!(children<&str, Vec<&str>>, preceded!(ws!(tag!("->")), separated_nonempty_list_complete!(ws!(char!(',')), alpha)));
named!(parse_vertex<&str, (&str, u32, Option<Vec<&str>>)>, ws!(tuple!(alpha, weight, opt!(complete!(children)))));

struct Program {
    name: String,
    weight: u32,
}

#[derive(Debug, Clone, Copy)]
struct Weight {
    weight : u32,
    extra : u32,
}

impl Weight {
    fn total(&self) -> u32 { self.weight + self.extra }
}

#[derive(Debug)]
enum BalanceResult {
    Balanced(Weight),
    Unbalanced(u32),
}

use BalanceResult::{Balanced, Unbalanced};

type ProgramTree = Graph<Program, ()>;

fn check_balanced(tree : &ProgramTree, root : NodeIndex) -> BalanceResult {
    let weight = tree[root].weight;
    let mut neighbors = tree.neighbors(root).peekable();
    if neighbors.peek().is_none() {
        return Balanced(Weight { weight, extra: 0 });
    }
    let mut neighbor_weights = Vec::new();
    for n in neighbors {
        match check_balanced(tree, n) {
            ub@Unbalanced(_) => return ub,
            Balanced(weight) => neighbor_weights.push(weight),
        }
    }
    if neighbor_weights.len() < 2 {
        let extra = neighbor_weights.get(0).map_or(0, Weight::total) + neighbor_weights.get(1).map_or(0, Weight::total);
        return Balanced(Weight { weight, extra });
    }
    let mut primary = neighbor_weights[0];
    let mut secondary = neighbor_weights[1];
    let mut primary_weight = primary.total();
    for &w in neighbor_weights[2..].iter() {
        if w.total() != primary_weight {
            if w.total() == secondary.total() {
                secondary = primary;
                primary = w;
                primary_weight = w.total();
            } else {
                secondary = w;
            }
        }
    }
    if secondary.total() != primary_weight {
        Unbalanced((secondary.weight + primary_weight) - secondary.total())
    } else {
        Balanced(Weight { weight, extra: primary_weight * (neighbor_weights.len() as u32) })
    }
}

fn get_root(tree : &ProgramTree) -> Option<NodeIndex> {
    if let Some(prog) = tree.node_indices().next() {
        let mut root = prog;
        while let Some(prog) = tree.neighbors_directed(root, Direction::Incoming).next() {
            root = prog;
        };
        Some(root)
    } else {
        None
    }
}

fn main() {
    let mut input = String::new();

    let mut nodes = BTreeMap::<String, NodeIndex>::new();
    let mut tree = ProgramTree::new();
    println!("Enter tree:");
    while let Ok(_) = io::stdin().read_line(&mut input) {
        {
            let input = input.trim_right();
            if input.is_empty() {
                if let Some(root) = get_root(&tree) {
                    println!("Root: {}", tree[root].name);
                    println!("{:?}\n", check_balanced(&tree, root));
                }
                nodes.clear();
                tree.clear();
                println!("Enter tree:");
            } else if let Ok((name, weight, children)) = parse_vertex(input).to_result() {
                {
                    let node = nodes.entry(String::from(name)).or_insert_with(|| tree.add_node(Program { name: String::from(name), weight }));
                    tree[*node].weight = weight;
                }
                if let Some(ref children) = children {
                    for &c in children {
                        nodes.entry(String::from(c)).or_insert_with(|| tree.add_node(Program { name: String::from(c), weight: 0 }));
                        tree.add_edge(nodes[name], nodes[c], ());
                    }
                }
            }
        }
        input.clear();
    }
}
