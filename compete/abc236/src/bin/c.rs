use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
        t: [String; m],
    }
    let mut set = HashSet::new();
    for x in t {
        set.insert(x);
    }
    for x in s {
        let yes = set.contains(&x);
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
