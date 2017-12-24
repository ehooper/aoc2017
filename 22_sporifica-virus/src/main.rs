extern crate util;
extern crate termion;
#[cfg(feature = "visualization")]
extern crate clap;

#[derive(Clone)]
struct Grid {
    width : usize,
    height : usize,
    grid : Vec<u8>,
}

fn run_simulation<S, P>(grid : Grid, iterations : usize, mut step : S, process : P) -> usize
where S : FnMut(&mut u8, &mut isize, &mut isize, &mut usize),
      P : Fn(&[u8], usize, usize) {
          if grid.width < 2 || grid.height < 2 {
              return 0;
          }
          let mut width = grid.width as isize;
          let mut height = grid.height as isize;
          let mut grid = grid.grid;

          let mut infections = 0;
          let mut index : (isize, isize) = (width / 2, height / 2);
          let (mut x, mut y) : (isize, isize) = (0, -1);
          for _ in 0..iterations {
              {
                  let node = unsafe { grid.get_unchecked_mut((index.1 * width + index.0) as usize) };
                  step(node, &mut x, &mut y, &mut infections);
              }
              index = (index.0 + x, index.1 + y);
              if index.0 < 0 || index.0 >= width || index.1 < 0 || index.1 >= height {
                  let new_width = 3 * width / 2;
                  let new_height = 3 * height / 2;
                  let dc = if index.0 < width / 2 { new_width - width } else { 0 };
                  let dr = if index.1 < height / 2 { new_height - height } else { 0 };
                  grid.resize((new_width * new_height) as usize, b'.');
                  for row in (0..height).rev() {
                      for col in (0..width).rev() {
                          grid.swap((row * width + col) as usize, ((row + dr) * new_width + (col + dc)) as usize);
                      }
                  }
                  index.0 += dc;
                  index.1 += dr;
                  width = new_width;
                  height = new_height;
              }
              process(&grid, width as usize, height as usize);
          }
          infections
      }

fn step(node : &mut u8, x : &mut isize, y : &mut isize, infections : &mut usize) {
    if *node == b'#' {
        *node = b'.';
        *y *= -1;
        std::mem::swap(x, y);
    } else {
        *infections += 1;
        *node = b'#';
        *x *= -1;
        std::mem::swap(x, y);
    }
}

fn step_evolved(node : &mut u8, x : &mut isize, y : &mut isize, infections : &mut usize) {
    match *node {
        b'.' => {
            *node = b'W';
            *x *= -1;
            std::mem::swap(x, y);
        },
        b'#' => {
            *node = b'F';
            *y *= -1;
            std::mem::swap(x, y);
        },
        b'W' => {
            *infections += 1;
            *node = b'#';
        },
        b'F' => {
            *node = b'.';
            *x *= -1;
            *y *= -1;
        },
        _ => { }
    }
}

fn simulate<S>(grid : Grid, iterations : usize, step : S) -> usize
where S : FnMut(&mut u8, &mut isize, &mut isize, &mut usize) {
    run_simulation(grid, iterations, step, |_, _, _| { })
}

#[cfg(feature = "visualization")]
fn print_grid(grid : &[u8], width : usize, height : usize) {
    use std::io::prelude::*;
    use termion::raw::IntoRawMode;

    let stdout = std::io::stdout().into_raw_mode().unwrap();
    let mut handle = stdout.lock();
    write!(handle, "{}", termion::cursor::Goto(1, 2)).unwrap();
    for i in 0..height {
        for j in 0..width {
            write!(handle, "{}", grid[i * width + j] as char).unwrap();
        }
        write!(handle, "\r\n").unwrap();
    }
    std::thread::sleep(std::time::Duration::from_millis(33));
}

#[cfg(feature = "visualization")]
fn simulate_print<S>(grid : Grid, iterations : usize, step : S) -> usize
where S : FnMut(&mut u8, &mut isize, &mut isize, &mut usize) {
    use termion::raw::IntoRawMode;
    use termion::{clear, color, cursor};

    // separate thread for capturing input (to exit cleanly)
    let stdout = std::io::stdout().into_raw_mode().unwrap();
    let input_thread = std::thread::spawn(move || {
        use std::io::{Read, Write};
        write!(stdout.lock(), "{hide}{clear_all}{top}{fg_color}{bg_color}press any key to exit{clear}{fg_reset}{bg_reset}",
        hide = cursor::Hide,
        clear_all = clear::All,
        top = cursor::Goto(1, 1),
        fg_color = color::Fg(color::White),
        bg_color = color::Bg(color::Cyan),
        clear = clear::UntilNewline,
        fg_reset = color::Fg(color::Reset),
        bg_reset = color::Bg(color::Reset)
        ).unwrap();
        std::io::stdin().bytes().next();
        write!(stdout.lock(), "{}{}", cursor::Show, clear::All).unwrap();
        stdout.lock().flush().unwrap();
        std::mem::drop(stdout);
        std::process::exit(0);
    });
    let (screen_w, screen_h) = termion::terminal_size().unwrap();
    let result = run_simulation(grid, iterations, step, |grid, width, height|
                                print_grid(grid, width.min(screen_w as usize), height.min(screen_h as usize - 2))
                               );
    input_thread.join().unwrap();
    result
}

fn parse_input(input : &str) -> Grid {
    let mut grid = Vec::new();
    let width = input.lines().next().unwrap_or("").len();
    let mut height = 0;
    for line in input.lines() {
        if line.len() == width {
            grid.extend_from_slice(line.as_bytes());
        }
        height += 1;
    }
    if width < 2 || height < 2 {
        eprintln!("warning: grid too small ({} x {})", width, height);
    }
    Grid { width, height, grid }
}

fn main() {
    #[cfg(feature = "visualization")]
    {
        use clap::{App, Arg};
        let options = App::new("Day 22: Sporifica Virus")
            .about("\nSolves the puzzle by default, or runs a visualization of the solution.")
            .arg(Arg::with_name("vis")
                 .long("visual")
                 .value_name("PART")
                 .possible_values(&["part1", "part2"])
                )
            .get_matches();
        if let Some(part) = options.value_of("vis") {
            if ! util::is_tty() {
                eprintln!("no TTY device");
                return;
            }
            println!("enter grid:");
            let mut input = String::new();
            util::get_multiline(&mut input).unwrap();
            let grid = parse_input(&input);
            if part == "part1" {
                simulate_print(grid, 10000, step);
            } else {
                simulate_print(grid, 10_000_000, step_evolved);
            }
            return;
        }
    }
    let run = |input : &str| {
        let grid = parse_input(input);
        println!("infections/original: {}", simulate(grid.clone(), 10000, step));
        println!("infections/evolved:  {}", simulate(grid.clone(), 10_000_000, step_evolved));
    };
    util::run_multiline("enter grid:", run)
}

#[test]
fn test_part_one() {
    let mut input =
        *b".........\
.........\
.........\
.....#...\
...#.....\
.........\
.........\
.........";
    let mut grid = Grid { width: 9, height: 8, grid: input.to_vec() };
    assert_eq!(5587, simulate(grid, 10000, step));
}

#[test]
fn test_part_two() {
    let mut input =
        *b".........\
.........\
.........\
.....#...\
...#.....\
.........\
.........\
.........";
    let mut grid = Grid { width: 9, height: 8, grid: input.to_vec() };
    assert_eq!(2511944, simulate(grid, 10_000_000, step_evolved));
}
