use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        _: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
        k: usize,
        cd: [(Usize1, Usize1); k],
    }

    let mut result = 0;
    for i in 0..(1 << k) {
        let mut set = HashSet::new();
        for (j, &(c, d)) in cd.iter().enumerate() {
            let j0 = if i & (1 << j) == 0 { c } else { d };
            set.insert(j0);
        }
        let r = ab
            .iter()
            .filter(|&(a, b)| set.contains(&a) && set.contains(&b))
            .count();
        result = result.max(r);
    }
    println!("{result}");
}
