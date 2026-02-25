use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        _n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut result = 0usize;
    let mut set = HashSet::new();
    for &ab in &ab {
        let (a, b) = ab;
        if set.contains(&(b, a)) {
            result += 1;
        }
        set.insert(ab);
    }
    println!("{result}");
}
