use std::collections::{BTreeMap, HashSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        mut a: [usize; n],
    }

    a.sort();

    if d == 0 {
        let result = a.windows(2).filter(|v| v[0] != v[1]).count() + 1;
        println!("{result}");
        return;
    }

    let mut m = BTreeMap::new();
    for &x in &a {
        *m.entry(x).or_insert(0usize) += 1;
    }

    let mut count_all = 0usize;

    let mut checked = HashSet::new();
    for (&x, _) in &m {
        if checked.contains(&x) {
            continue;
        }

        let mut x = x;
        let mut counts = [0usize, 0usize]; // 0番目からの偶数合計, 1番目からの奇数合計
        let mut j = 0;

        // 偶奇の出現数を調べて多い方を採用
        while let Some(&count) = m.get(&x) {
            counts[j] += count;
            checked.insert(x);
            j = (j + 1) % 2;
            x += d;
        }

        count_all += counts[0].max(counts[1]);
    }

    let result = n - count_all;
    println!("{result}");
}
