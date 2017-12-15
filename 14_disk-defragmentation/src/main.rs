extern crate util;
extern crate knot_hash;
extern crate petgraph;

use util::{is_tty, prompt};
use knot_hash::{KnotHash, Digest};

type MemoryMap = Box<[[u8; 16]; 128]>;

fn make_memory_map(khash : &mut KnotHash, input : &str) -> MemoryMap {
    use std::fmt::Write;

    let mut mem_map = Box::new([[0; 16]; 128]);
    let mut buffer = String::with_capacity(input.len() + 4);
    for (i, row) in mem_map.iter_mut().enumerate() {
        buffer.clear();
        buffer += input;
        write!(&mut buffer, "-{}", i).unwrap();
        let Digest(bytes) = khash.digest(buffer.as_bytes());
        *row = bytes;
    }
    mem_map
}

fn count_regions(mem_map : &MemoryMap) -> usize {
    use petgraph::unionfind::UnionFind;

    const SIZE : usize = 128 * 128;

    let mut block_index : usize = 0;
    let mut used : [bool; SIZE] = [false; SIZE];
    for &row in mem_map.iter() {
        for byte in &row {
            let mut slice = [false; 8];
            for (i, used) in slice.iter_mut().enumerate() {
                *used = byte & (1 << (7 - i)) != 0;
            }
            used[block_index..(block_index + 8)].copy_from_slice(&slice);
            block_index += 8;
        }
    };
    let mut regions = UnionFind::<u16>::new(SIZE);
    for block in used.iter().cloned().enumerate().filter(|&(_, is_used)| is_used).map(|(b, _)| b) {
        let row = block / 128;
        let col = block % 128;
        if col < 127 && used[block + 1] {
            regions.union(block as u16, (block + 1) as u16);
        }
        if row < 127 && used[block + 128] {
            regions.union(block as u16, (block + 128) as u16);
        }
    }
    let mut colored_regions = regions.into_labeling();
    colored_regions.retain(|&block| used[block as usize]);
    colored_regions.sort();
    colored_regions.dedup();
    colored_regions.len()
}

fn count_used(mem_map : &MemoryMap) -> u32 {
    mem_map.iter().map(|row| -> u32 { row.iter().cloned().map(u8::count_ones).sum() }).sum()
}

fn main() {
    use std::io::Read;

    let mut input = String::new();
    let mut khash = KnotHash::new();

    let mut run = |input : &str| {
        let mem_map = make_memory_map(&mut khash, input);
        println!("blocks used: {}", count_used(&mem_map));
        println!("regions:     {}", count_regions(&mem_map));
    };

    if ! is_tty() {
        if std::io::stdin().read_to_string(&mut input).is_ok() {
            run(&input);
        }
        return;
    }
    prompt();
    while std::io::stdin().read_line(&mut input).is_ok() {
        run(input.trim_right());
        input.clear();
        prompt();
    }
}

#[test]
fn test_example() {
    let input = "flqrgnkx";
    let mem_map = make_memory_map(&mut KnotHash::new(), input);
    assert_eq!(8108, count_used(&mem_map));
    assert_eq!(1242, count_regions(&mem_map));
}
