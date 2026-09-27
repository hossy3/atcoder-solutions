use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }

    let mut set = BTreeSet::new();
    for a in a {
        set.insert(a);
    }

    let mut result = 0usize;
    let mut cur = 0isize;
    while set.len() > 0 {
        let x0 = set.range(..cur).last();
        let x1 = set.range(cur..).next();
        let x = if x0.is_none() {
            *x1.unwrap()
        } else if x1.is_none() {
            *x0.unwrap()
        } else {
            let x0 = *x0.unwrap();
            let x1 = *x1.unwrap();
            if cur - x0 <= x1 - cur { x0 } else { x1 }
        };
        result += cur.abs_diff(x);
        cur = x;
        set.remove(&x);
    }

    println!("{result}");
}
