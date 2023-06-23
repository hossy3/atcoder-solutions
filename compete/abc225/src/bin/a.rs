use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let (a, b, c) = (s[0], s[1], s[2]);
    let mut set = HashSet::new();
    set.insert((a, b, c));
    set.insert((a, c, b));
    set.insert((b, a, c));
    set.insert((b, c, a));
    set.insert((c, a, b));
    set.insert((c, b, a));
    let result = set.len();
    println!("{}", result);
}
