use std::{cmp::Reverse, collections::BTreeSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut set = BTreeSet::new();
    for (i, &num) in a.iter().enumerate() {
        set.insert((Reverse(num), i));
    }

    let mut result = 0usize;
    while set.len() >= k {
        result += 1;
        let mut set0 = BTreeSet::new();
        for _ in 0..k {
            let &x = set.iter().next().unwrap();
            set.remove(&x);
            let (Reverse(num), i) = x;
            if num > 1 {
                set0.insert((Reverse(num - 1), i));
            }
        }
        set.append(&mut set0);
    }

    println!("{}", result);
}
