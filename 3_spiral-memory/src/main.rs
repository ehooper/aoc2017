use std::io;
use std::io::Write;

fn main() {
    let mut input = String::new();
    print!("> ");
    io::stdout().flush().unwrap();
    while let Ok(_) = io::stdin().read_line(&mut input) {
        let num : i32 = input.trim_right().parse().unwrap();
        let ring : i32 = (f64::from(num).sqrt().ceil() as i32) / 2;
        let mut ring_min = (ring - 1) * 2 + 1;
        ring_min = ring_min * ring_min + 1;
        let m1 = ring_min + (ring - 1);
        let m2 = m1 + (2 * ring);
        let m3 = m1 + (4 * ring);
        let m4 = m1 + (6 * ring);

        //println!("\t{}\n{}\t{}\t{}\n\t{}", m2, m3, ring, m1, m4);

        let mut hd = ring;
        hd = i32::min(hd, i32::abs(num - m1));
        hd = i32::min(hd, i32::abs(num - m2));
        hd = i32::min(hd, i32::abs(num - m3));
        hd = i32::min(hd, i32::abs(num - m4));

        let steps = ring + hd;

        input.clear();
        println!("{}", steps);
        print!("> ");
        io::stdout().flush().unwrap();
    }
}
