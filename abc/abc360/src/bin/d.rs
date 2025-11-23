use std::iter;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        t: i64,
        s: Chars,
        x: [i64; n],
    }

    // アリの初期位置を並び替える
    let mut vl = vec![]; // 左向き
    let mut vr = vec![]; // 右向き
    for (c, pos) in iter::zip(s, x) {
        if c == '0' {
            vl.push(pos);
        } else {
            vr.push(pos);
        }
    } 
    vl.sort();
    vr.sort();

    // 初期状態で r が l の左にどれだけいるか調べる
    let mut count0 = 0usize;
    for &l in &vl {
        count0 += vr.partition_point(|&r| r < l);
    }

    // 時間経過する
    for l in vl.iter_mut() {
        *l -= t;
    }
    for r in vr.iter_mut() {
        *r += t;
    }

    // 時間経過後に r が l の左にどれだけいるか調べる
    let mut count1 = 0usize;
    for &l in &vl {
        count1 += vr.partition_point(|&r| r < l);
    }

    let result = count0 - count1;
    println!("{result}");
}
