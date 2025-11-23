use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::input_interactive;

fn main() {
    // 最初に N,M,L,U が標準入力から与えられる。
    input_interactive! {
        n: usize,
        m: usize,
        l: isize,
        u: isize,
    }

    // i 番目のカードに書き込む数を A​_i として、以下のように標準出力に出力せよ。
    let width = u - l;

    // 最初 m 個は底上げ用
    let mut a = vec![l; m];
    // 残りは隙間を埋めるように
    const K: f64 = 16_777_216.0 / 4.0;
    for i in 0..(n - m) {
        let x =
            ((width as f64 / K) * 2_f64.powf(K.log2() * (i as f64) / ((n - m) as f64))) as isize;
        a.push(x);
    }
    a.sort();
    a.reverse();

    println!("{}", a.iter().join(" "));

    // その後、B_1​,…,B_M が標準入力から与えられる。
    input_interactive! {
        b: [isize; m],
    }

    // i 番目のカードを捨てるなら X_i =0、そうでないなら i 番目のカードが属する山を X_i として、
    // 以下のように標準出力に出力せよ。
    let mut bb = vec![(0, vec![]); m + 1]; // 採用した番号の配列。0 は捨てる
    for (i, b) in b.iter().enumerate() {
        bb[i + 1].0 = *b; // 残りこれだけ入れられる
    }

    for (i, &a) in a.iter().enumerate() {
        // 置けるなかで一番隙間が小さくなるところに置く
        let mut best_j = 0;
        let mut best_diff = isize::MAX;
        for j in 0..m {
            let diff = bb[j + 1].0 - a;
            if diff >= 0 && diff < best_diff {
                best_diff = diff;
                best_j = j + 1;
            }
        }
        bb[best_j].0 -= a;
        bb[best_j].1.push(i);
    }

    // はみ出した方がスコアが高い場合があるので、調整する
    let mut queue = BinaryHeap::new();
    for &i in &bb[0].1 {
        queue.push((Reverse(a[i]), i));
    }
    bb[0].1.clear();

    while let Some((Reverse(x), i)) = queue.pop() {
        // 置けるなかで一番隙間が小さくなるところに置く
        let mut best_j = 0;
        let mut best_count = 0;
        let mut best_diff = isize::MAX;
        for j in 0..m {
            let mut sum = x; // 今回置く
            let diff = (b[j] - sum).abs();
            if diff < bb[j + 1].0.abs() && diff < best_diff {
                best_diff = diff;
                best_j = j + 1;
                best_count = 0;
            }
            for (k0, &k) in bb[j + 1].1.iter().enumerate() {
                sum += a[k];
                let diff = (b[j] - sum).abs();
                if diff < bb[j + 1].0.abs() && diff < best_diff {
                    best_diff = diff;
                    best_j = j + 1;
                    best_count = k0 + 1;
                }
            }
        }

        if best_j == 0 {
            bb[0].1.push(i);
            continue;
        }

        while bb[best_j].1.len() > best_count {
            let j = bb[best_j].1.pop().unwrap();
            queue.push((Reverse(a[j]), j));
        }
        while let Some(j) = bb[0].1.pop() {
            queue.push((Reverse(a[j]), j));
        }

        bb[best_j].1.push(i);
        bb[best_j].1.sort_by_cached_key(|&x| -a[x]);
        let mut rest = b[best_j - 1];
        for &j in &bb[best_j].1 {
            rest -= a[j];
        }
        bb[best_j].0 = rest;
    }

    let mut results = vec![0; n];
    for (i, (_, v)) in bb.iter().enumerate() {
        for &j in v {
            results[j] = i;
        }
    }

    println!("{}", results.iter().join(" "));
}
