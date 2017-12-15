pub struct Digest(pub [u8; 16]);

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

pub struct KnotHash {
    index : usize,
    skip : usize,
    byte_table : [u8; 256],
}

impl Default for KnotHash {
    fn default() -> KnotHash {
        let mut byte_table = [0; 256];
        reset_byte_table(&mut byte_table);
        KnotHash {
            index: 0,
            skip: 0,
            byte_table,
        }
    }
}

impl KnotHash {
    pub fn new() -> KnotHash { Default::default() }

    pub fn byte_table(&self) -> &[u8; 256] { &self.byte_table }

    pub fn reset(&mut self) {
        self.index = 0;
        self.skip = 0;
        reset_byte_table(&mut self.byte_table);
    }

    pub fn round(&mut self, input : &[u8], pad : &[u8]) {
        for &len in input.iter().chain(pad) {
            let len = len as usize;
            reverse(&mut self.byte_table, self.index, len);
            self.index = (self.index + len + self.skip) % 256;
            self.skip += 1;
        }
    }

    pub fn digest(&mut self, input : &[u8]) -> Digest {
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

#[test]
fn test_part_one() {
    let mut input = [0, 1, 2, 3, 4];
    let mut index = 0;
    for (skip, len) in [3, 4, 1, 5].iter().cloned().enumerate() {
        reverse(&mut input, index, len);
        index = (index + len + skip) % 5;
    }
    assert_eq!([3, 4, 2, 1, 0], input);
}

#[test]
fn test_part_two() {
    let mut knot_hash = KnotHash::new();
    assert_eq!("a2582a3a0e66e6e86e3812dcb672a272", knot_hash.digest(b"").to_string());
    assert_eq!("33efeb34ea91902bb2f59c9920caa6cd", knot_hash.digest(b"AoC 2017").to_string());
    assert_eq!("3efbe78a8d82f29979031a4aa0b16a9d", knot_hash.digest(b"1,2,3").to_string());
    assert_eq!("63960835bcdc130f0b66d7ff4f6a5a8e", knot_hash.digest(b"1,2,4").to_string());
}
