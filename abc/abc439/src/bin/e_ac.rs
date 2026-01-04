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
    let mut state = vec![0usize]; // 長さ0
    for (_, v) in &map_a {
        let mut v0 = vec![];
        for &x in v {
            v0.push(v_b[x]);
        }
        v0.sort();
        v0.reverse();

        for x in v0 {
            let j = state.partition_point(|&x0| x0 < x);
            if j == state.len() {
                state.push(x);
            } else {
                state[j] = x;
            }
        }
    }

    let result = state.len() - 1;
    println!("{result}");
}
