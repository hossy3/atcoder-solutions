use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        _n: usize,
        m: usize,
        rc: [(usize, usize); m],
    }

    let mut set = HashSet::new();
    for &(r, c) in &rc {
        if set.contains(&(r, c))
            || set.contains(&(r + 1, c))
            || set.contains(&(r, c + 1))
            || set.contains(&(r + 1, c + 1))
        {
            continue;
        }
        set.insert((r, c));
        set.insert((r + 1, c));
        set.insert((r, c + 1));
        set.insert((r + 1, c + 1));
    }
    println!("{}", set.len() / 4);
}
