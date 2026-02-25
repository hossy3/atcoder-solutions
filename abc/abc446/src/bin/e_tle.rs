use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        m: usize,
        a: usize,
        b: usize,
    }

    let mut result = 0usize;
    for x in 0..m {
        for y in 0..m {
            let mut set = HashSet::new();
            let mut x = x % m;
            let mut y = y % m;
            if x == 0 || y == 0 {
                continue;
            }
            set.insert((x, y));

            loop {
                (x, y) = (y, (a * y + b * x) % m);
                if y == 0 {
                    break;
                }
                if !set.insert((x, y)) {
                    result += 1;
                    break;
                }
            }
        }
    }

    println!("{result}");
}
