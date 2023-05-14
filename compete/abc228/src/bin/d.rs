use std::collections::{BTreeSet, HashMap};

use proconio::input;

fn main() {
    input! {
        q: usize,
    }
    const MOD: usize = 1 << 20;
    let mut set: BTreeSet<_> = (0..MOD).into_iter().collect();
    let mut map = HashMap::new();

    for _ in 0..q {
        input! {
            t: usize,
            x: usize,
        }
        let x0 = x % MOD;
        if t == 1 {
            if let Some(&y) = set.range(x0..).next() {
                map.insert(y, x);
                set.remove(&y);
            } else if let Some(&y) = set.range(0..).next() {
                map.insert(y, x);
                set.remove(&y);
            }
        } else {
            if let Some(&result) = map.get(&x0) {
                println!("{}", result);
            } else {
                println!("{}", -1);
            }
        }
    }
}
