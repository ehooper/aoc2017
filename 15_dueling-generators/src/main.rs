fn main() {
    let mut args = std::env::args();
    let a = str::parse(&args.nth(1).expect("first argument missing")).expect("could not parse a");
    let b = str::parse(&args.next().expect("second argument missing")).expect("could not parse b");
    println!("{}", matching_pairs(a, b, 40_000_000));
    println!("{}", matching_pairs_2(a, b, 5_000_000));
}

fn matching_pairs(mut a : u64, mut b : u64, iterations : usize) -> usize {
    (0..iterations).filter(|_| {
        a = (a * 16807) % 2147483647;
        b = (b * 48271) % 2147483647;
        a & 0xffff == b & 0xffff
    }).count()
}

fn matching_pairs_2(mut a : u64, mut b : u64, iterations : usize) -> usize {
    (0..iterations).filter(|_| {
        a = (a * 16807) % 2147483647;
        while a % 4 != 0 {
            a = (a * 16807) % 2147483647;
        }
        b = (b * 48271) % 2147483647;
        while b % 8 != 0 {
            b = (b * 48271) % 2147483647;
        }
        a & 0xffff == b & 0xffff
    }).count()
}

#[test]
fn test_part_one() {
    assert_eq!(588, matching_pairs(65, 8921, 40_000_000));
    assert_eq!(309, matching_pairs_2(65, 8921, 5_000_000));
}
