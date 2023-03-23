use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }

    let mut set = BTreeSet::new();
    set.insert(0);
    for a in a {
        let prev = set.clone();
        for x in prev {
            let x = x + a;
            if x <= s {
                set.insert(x);
            }
        }
    }

    let yes = set.contains(&s);
    println!("{}", if yes { "Yes" } else { "No" });
}
