use ac_library::{Additive, Segtree};
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Usize1; n],
        q: usize,
        x: [Usize1; q],
    }

    // a の出現回数
    let mut v = vec![0usize; m];
    for &x in &a {
        v[x] += 1;
    }
    let mut v0 = vec![vec![]; n + 1];
    for (i, &x) in v.iter().enumerate() {
        v0[x].push(i); // x個存在する
    }
    while let Some(v) = v0.last() {
        if v.len() > 0 {
            break;
        }
        v0.pop();
    }

    let mut results = vec![0; q];

    // 数字を追加しなくて良いものは先に回答する
    for (i, &x) in x.iter().enumerate() {
        if x < n {
            results[i] = a[x] + 1;
        }
    }

    // x をクエリ先読み
    let xi = x
        .iter()
        .enumerate()
        .filter(|&(_, &x)| x >= n)
        .map(|(i, &x)| (x - n, i))
        .sorted()
        .collect::<Vec<_>>();

    let mut segtree: Segtree<Additive<_>> = vec![0usize; m].into(); // 現在の対象
    for &x in &v0[0] {
        segtree.set(x, 1);
    }
    let mut count = v0[0].len(); // 追加する候補の個数
    let mut len = 0; // len 以下の個数が最初に入っているものが対象
    let mut j = 0;

    'outer: for (i0, &(x, i)) in xi.iter().enumerate() {
        let mut x0 = if i0 == 0 { x } else { x - xi[i0 - 1].0 }; // 前回との差

        // 全部使う
        if len == v0.len() - 1 {
            j = (j + x0) % m;
            results[i] = j + 1;
            continue;
        }

        // len を変えずに処理できる場合
        if j + x0 < count {
            j += x0;
            results[i] = segtree.max_right(0, |&v| v <= j) + 1;
            continue;
        }

        x0 -= count - j;
        j = 0;
        len += 1;
        count += v0[len].len();
        for &x in &v0[len] {
            segtree.set(x, 1);
        }
        while len < v0.len() - 1 {
            if j + x0 < count {
                j += x0;
                results[i] = segtree.max_right(0, |&v| v <= j) + 1;
                continue 'outer;
            }
            x0 -= count;
            len += 1;
            count += v0[len].len();
            for &x in &v0[len] {
                segtree.set(x, 1);
            }
        }

        // 全部使う
        j = (j + x0) % m;
        results[i] = j + 1;
    }

    for result in results {
        println!("{result}");
    }
}
