use proconio::{input, marker::Usize1};
use std::collections::BTreeMap;

fn f(mut ab: Vec<(usize, usize)>) -> bool {
    let n = ab.len();

    for i in 0..n {
        if ab[i].0 > ab[i].1 {
            ab[i] = (ab[i].1, ab[i].0);
        }
    }

    let mut map = BTreeMap::new();
    for &(a, b) in &ab {
        map.insert(a, (b, n));
    }

    let mut level = 0usize;
    while let Some((_, (x, y))) = map.pop_first() {
        if y == n {
            map.insert(x, (0, level));
            level += 1;
        } else {
            if level == 0 || level - 1 != y {
                return true;
            }
            level -= 1;
        }
    }
    false
}

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n],
    }
    let yes = f(ab);
    println!("{}", if yes { "Yes" } else { "No" });
}
