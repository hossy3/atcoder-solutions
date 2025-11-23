use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: i64,
        m: usize,
        ab: [(i64, i64); m],
    }

    let mut result = n * n;
    let mut set = HashSet::new();
    let d = [
        (2, 1),
        (1, 2),
        (-1, 2),
        (-2, 1),
        (-2, -1),
        (-1, -2),
        (1, -2),
        (2, -1),
    ];
    for (a, b) in ab {
        if set.insert((a, b)) {
            result -= 1;
        }
        for &(a0, b0) in &d {
            let (a, b) = (a + a0, b + b0);
            if a <= 0 || a > n || b <= 0 || b > n {
                continue;
            }
            if set.insert((a, b)) {
                result -= 1;
            }
        }
    }
    println!("{result}");
}
