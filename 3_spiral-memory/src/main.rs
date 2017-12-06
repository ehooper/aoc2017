use std::io;

/* S2 .. S2 S1
 * S3       .
 * .        .
 * .        S1
 * S3 S4 .. S4
 */
fn spiral_to_cart(i : i32) -> (i32, i32) {
    if i == 1 {
        return (0, 0);
    }
    let ring : i32 = (f64::from(i).sqrt().ceil() as i32) / 2;
    let ring_len = 2 * ring - 1;
    let ring_min = ring_len * ring_len;
    let seg_len = ring_len + 1;
    if i <= ring_min + seg_len {
        let v = i - ring_min;
        return (ring, v - ring);
    }
    if i <= ring_min + 2 * seg_len {
        let v = i - ring_min - seg_len;
        return (ring - v, ring);
    }
    if i <= ring_min + 3 * seg_len {
        let v = i - ring_min - 2 * seg_len;
        return (-ring, ring - v);
    }
    assert!(i <= ring_min + 4 * seg_len);
    let v = i - ring_min - 3 * seg_len;
    (v - ring, -ring)
}

fn cart_to_spiral(i : (i32, i32)) -> i32 {
    if i == (0, 0) {
        return 1;
    }
    let (x, y) = i;
    let ring = i32::max(x.abs(), y.abs());
    let ring_min = (2 * ring - 1).pow(2);
    if x == ring && y > -ring {
        return ring_min + y + ring;
    }
    if y == ring && x < ring {
        return ring_min + 2 * ring + (ring - x);
    }
    if x == -ring && y < ring {
        return ring_min + 4 * ring + (ring - y);
    }
    assert!(y == -ring && x > -ring);
    ring_min + 6 * ring + (x + ring)
}

fn taxicab_distance(p : (i32, i32)) -> i32 {
    p.0.abs() + p.1.abs()
}

fn adjacent_sum(limit : i32) -> i32 {
    if limit <= 1 {
        return 1;
    }
    let mut memo : Vec<i32> = vec!(1);
    let mut address = 2;
    loop {
        let (x, y) = spiral_to_cart(address);
        let neighbors : [i32; 8] = [
            cart_to_spiral((x - 1, y - 1)),
            cart_to_spiral((x - 1, y)),
            cart_to_spiral((x - 1, y + 1)),
            cart_to_spiral((x, y - 1)),
            cart_to_spiral((x, y + 1)),
            cart_to_spiral((x + 1, y - 1)),
            cart_to_spiral((x + 1, y)),
            cart_to_spiral((x + 1, y + 1)),
        ];
        let sum : i32 = neighbors.iter().map(
            |&i| if i < address { memo[(i - 1) as usize] } else { 0 }
            ).sum();

        if sum > limit {
            return sum;
        }

        memo.push(sum);
        address += 1;
    }
}

fn prompt() {
    use std::io::Write;

    print!("> ");
    io::stdout().flush().unwrap();
}

fn main() {
    let arg = std::env::args().nth(1);
    let part2 = arg.map_or(false, |s| s == "--part2");

    let mut input = String::new();
    prompt();
    while let Ok(_) = io::stdin().read_line(&mut input) {
        let num : i32 = input.trim_right().parse().unwrap();

        if part2 {
            let sum = adjacent_sum(num);
            println!("{}", sum);
        } else {
            let steps = taxicab_distance(spiral_to_cart(num));
            println!("{}", steps);
        }

        input.clear();
        prompt();
    }
}
