use std::collections::BTreeSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [usize; n],
        p: [usize; n],
        lr: [(Usize1, Usize1); m],
    }

    let mut set = BTreeSet::new(); // 配布できないプラン
    for i in 0..n {
        if s[i] < p[i] {
            set.insert(i);
        }
    }

    for &(l, r) in &lr {
        let yes = set.range(l..=r).next().is_none();
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
