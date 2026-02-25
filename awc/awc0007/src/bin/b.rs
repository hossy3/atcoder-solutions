use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut sets = vec![];
    for _ in 0..n {
        input! {
            m: usize,
            w: [String; m],
        }
        let mut set = HashSet::new();
        for s in w {
            set.insert(s);
        }
        sets.push(set);
    }

    let mut result = 0;
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            if sets[i].intersection(&sets[j]).count() >= k {
                result += 1;
            }
        }
    }
    println!("{result}");
}
