use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
    }

    a.sort();
    let mut set = BTreeSet::new();
    for i in 0..=k {
        set.insert(a[0] * i);
    }

    for x in &a[1..] {
        if x >= set.iter().last().unwrap() {
            break;
        }

        let s = set.clone();
        for &y in &s {
            for i in 1..k {
                let value = y + x * i;
                let max = *set.iter().last().unwrap();
                if value >= max {
                    break;
                }
                if !set.contains(&value) {
                    set.remove(&max);
                    set.insert(value);
                }
            }
        }
    }

    let result = set.iter().last().unwrap();
    println!("{}", result);
}
