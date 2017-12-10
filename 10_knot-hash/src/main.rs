fn reverse(a : &mut [u8], index : usize, len : usize) {
    let size = a.len();
    let mut start = index % size;
    let mut end = ((start + len).wrapping_sub(1)) % size;
    for _ in 0..(len/2)  {
        a.swap(start, end);
        if end == 0 {
            end = size;
        }
        end -= 1;
        start += 1;
        if start == size {
            start = 0;
        }
    }
}

struct Digest([u8; 16]);

impl std::fmt::Display for Digest {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

fn reset_byte_table(byte_table : &mut [u8; 256]) {
    for (i, b) in byte_table.iter_mut().enumerate() { *b = i as u8 };
}

struct KnotHash<'a> {
    byte_table : &'a mut [u8; 256],
    size : usize,
    index : usize,
    skip : usize,
}

impl<'a> KnotHash<'a> {
    fn new(byte_table : &'a mut [u8; 256], size : usize) -> KnotHash<'a> {
        reset_byte_table(byte_table);
        KnotHash {
            byte_table,
            size,
            index: 0,
            skip: 0,
        }
    }

    fn reset(&mut self) {
        self.index = 0;
        self.skip = 0;
        reset_byte_table(self.byte_table);
    }

    fn round(&mut self, input : &[u8], pad : &[u8]) {
        for &len in input.iter().chain(pad) {
            let len = len as usize;
            reverse(&mut self.byte_table[0..self.size], self.index, len);
            self.index = (self.index + len + self.skip) % self.size;
            self.skip += 1;
        }
    }

    fn digest(&mut self, input : &[u8]) -> Digest {
        self.reset();
        for _ in 0..64 {
            self.round(input, &[17, 31, 73, 47, 23]);
        }
        let mut digest : [u8; 16] = [0; 16];
        for (val, d) in self.byte_table.chunks(16).map(|chunk| chunk.iter().fold(0, |a, b| a ^ b)).zip(&mut digest) {
            *d = val;
        }
        Digest(digest)
    }
}

fn prompt() {
    use std::io::Write;

    print!("> ");
    std::io::stdout().flush().unwrap();
}

fn run_part1(mut byte_table : &mut [u8; 256]) {
    let mut input = String::new();

    macro_rules! reset {
        () => { input.clear(); prompt(); }
    }

    let mut size = 256;
    while let Ok(_) = std::io::stdin().read_line(&mut input) {
        // For part one you can set the table size to 5 by entering `size 5` to test the example.
        // To set it back, obviously, enter `size 256`.
        if input.to_lowercase().starts_with("size") {
            if let Some(Ok(new_size)) = input.split_whitespace().nth(1).map(str::parse::<usize>) {
                if new_size == 0 {
                    eprintln!("size must be greater than 0");
                } else if new_size > 256 {
                    eprintln!("size too large (max 256): {}", new_size);
                } else {
                    size = new_size;
                    println!("input size = {}", size);
                }
                reset!();
                continue;
            }
        }

        let lengths : Result<Vec<u8>, _> = input.split(|c : char| c == ',' || c.is_whitespace())
            .filter(|s| ! s.is_empty())
            .map(str::parse::<u8>)
            .collect();

        if let Err(e) = lengths {
            eprintln!("error parsing input: {} '{}'", std::error::Error::description(&e), input.trim_right());
            reset!();
            continue;
        }

        {
            let mut knot_hash = KnotHash::new(&mut byte_table, size);
            knot_hash.round(&lengths.unwrap(), &[]);
        }
        let result = (byte_table[0] as usize) * (byte_table[1] as usize);
        println!("{}", result);

        reset!();
    }
}

fn run_part2(mut byte_table : &mut [u8; 256]) {
    let mut knot_hash = KnotHash::new(&mut byte_table, 256);

    let mut input = String::new();
    while let Ok(_) = std::io::stdin().read_line(&mut input) {
        let digest = knot_hash.digest(input.trim().as_bytes());
        println!("{}", digest);
        input.clear();
        prompt();
    }
}

fn main() {
    let arg = std::env::args().nth(1);
    let part2 = arg.map_or(false, |s| s == "--part2");

    let mut byte_table : [u8; 256] = [0; 256];

    prompt();
    if part2 {
        run_part2(&mut byte_table);
    } else {
        run_part1(&mut byte_table);
    }
}

#[test]
fn test_part_one() {
    const TEST_SIZE : usize = 5;
    let mut byte_table : [u8; 256] = [0; 256];
    {
        let mut knot_hash = KnotHash::new(&mut byte_table, TEST_SIZE);
        knot_hash.round(&[3, 4, 1, 5], &[]);
    }
    assert_eq!([3, 4, 2, 1, 0], byte_table[0..TEST_SIZE]);
}

#[test]
fn test_part_two() {
    let mut byte_table = [0; 256];
    let mut knot_hash = KnotHash::new(&mut byte_table, 256);
    assert_eq!("a2582a3a0e66e6e86e3812dcb672a272", knot_hash.digest(b"").to_string());
    assert_eq!("33efeb34ea91902bb2f59c9920caa6cd", knot_hash.digest(b"AoC 2017").to_string());
    assert_eq!("3efbe78a8d82f29979031a4aa0b16a9d", knot_hash.digest(b"1,2,3").to_string());
    assert_eq!("63960835bcdc130f0b66d7ff4f6a5a8e", knot_hash.digest(b"1,2,4").to_string());
}
