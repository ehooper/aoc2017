extern crate util;

struct AsciiGrid {
    cols : usize,
    rows : usize,
    grid : Vec<u8>,
}

impl AsciiGrid {
    fn new() -> AsciiGrid {
        AsciiGrid { cols : 0, rows: 0, grid: Vec::new() }
    }

    fn get(&self, x : i32, y : i32) -> Option<u8> {
        if x < 0 || x >= (self.cols as i32) {
            return None;
        }
        if y < 0 || y >= (self.rows as i32) {
            return None;
        }
        Some(self.grid[(y as usize) * self.cols + (x as usize)])
    }

    fn add_row(&mut self, row : &[u8]) -> Result<(), String> {
        if self.rows == 0 {
            self.cols = row.len();
        } else if row.len() != self.cols {
            return Err(format!("invalid row size ({}) for row '{}' (expected {})", row.len(), std::str::from_utf8(row).unwrap(), self.cols));
        }
        self.grid.extend_from_slice(row);
        self.rows += 1;
        Ok(())
    }
}

fn parse_input(input : &str) -> Result<AsciiGrid, String> {
    let mut grid = AsciiGrid::new();
    for row in input.lines() {
        grid.add_row(row.as_bytes())?;
    }
    Ok(grid)
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::*;

impl Direction {
    fn apply(self, (x, y) : (i32, i32)) -> (i32, i32) {
        match self {
            Up    => (x, y - 1),
            Down  => (x, y + 1),
            Left  => (x - 1, y),
            Right => (x + 1, y),
        }
    }
}

struct Path { path : String, steps : usize }

fn follow_path(grid : &AsciiGrid) -> Path {
    let start = grid.grid[0..grid.cols].iter().position(|&c| c == b'|').unwrap_or(0);
    let (mut x, mut y) = (start as i32, 0_i32);
    let mut dir = Down;
    let mut path = String::new();
    let mut steps = 0;
    loop {
        match grid.get(x, y) {
            None | Some(b' ') => return Path { path, steps },
            Some(b'|') | Some(b'-') | Some(b'+') => { },
            Some(c) => path.push(c as char)
        }
        let (mut xn, mut yn) = dir.apply((x, y));
        match grid.get(xn, yn) {
            None | Some(b' ') => {
                let dirs = match dir {
                    Up   | Down  => [Left, Right],
                    Left | Right => [Up,   Down],
                };
                for d in dirs.iter().cloned() {
                    let (xd, yd) = d.apply((x, y));
                    match grid.get(xd, yd) {
                        None | Some(b' ') => { },
                        _ => { dir = d; xn = xd; yn = yd; break; }
                    }
                }
            },
            _ => {  }
        }
        steps += 1;
        x = xn;
        y = yn;
    }
}

fn main() {
    let run = |input : &str| {
        let grid = match parse_input(input) {
            Ok(g) => g,
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        };
        let path = follow_path(&grid);
        println!("path:  {}", path.path);
        println!("steps: {}", path.steps);
    };

    util::run_multiline("enter route:", run);
}

#[test]
fn test_example() {
    let input =
"     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ ";
    let grid = parse_input(input).unwrap();
    let path = follow_path(&grid);
    assert_eq!("ABCDEF", path.path);
    assert_eq!(38, path.steps);
}
