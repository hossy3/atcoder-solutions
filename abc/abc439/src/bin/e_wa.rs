use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }

    // 並び替える
    let mut map_a = BTreeMap::new();
    let mut map_b = BTreeMap::new();
    for (i, &(a, b)) in ab.iter().enumerate() {
        map_a.entry(a).or_insert(vec![]).push(i);
        map_b.entry(b).or_insert(vec![]).push(i);
    }

    // b のソートした順番を引けるようにする
    let mut v_b = vec![0usize; n];
    for (i, (_, v)) in map_b.iter().enumerate() {
        for &j in v {
            v_b[j] = i + 1;
        }
    }

    // DP
    let mut dp = vec![(0usize, 0usize)]; // 長さ0, 末尾0
    for (_, v) in &map_a {
        let mut v0 = vec![];
        for &x in v {
            v0.push(v_b[x]);
        }
        v0.sort();

        let mut dp0 = vec![];
        for &(len, x) in dp.iter().rev() {
            if dp0.len() == 0 {
                dp0.push((len, x));
            } else {
                let k = dp0.len() - 1;
                if dp0[k].0 == len && dp0[k].1 > x {
                    dp0[k].1 = x;
                }
            }

            let j = v0.partition_point(|&x0| x0 < x);
            if j < v0.len() {
                let j = v0[j];
                dp0.push((len + 1, j));
            }
        }

        if dp0.len() > 0 {
            dp = dp0;
        }
    }

    let result = dp[dp.len() - 1].0;
    println!("{result}");
}
