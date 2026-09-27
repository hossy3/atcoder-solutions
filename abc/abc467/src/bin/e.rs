use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: isize,
        a: [isize; n],
        b: [isize; n - 1],
    }

    // まず a[0] を固定して a[1] 以降を解決するような c を作る
    let mut c = vec![0; n];
    let mut diff = 0;
    let mut result = 0;
    for i in 0..(n - 1) {
        let a0 = (a[i] + a[i + 1] + diff) % m;
        diff = (b[i] + m - a0) % m;
        result += diff;
        c[i + 1] = diff;
    }

    // c を偶数と奇数に分け、map に登録する
    let mut map = BTreeMap::new();
    for i in 0..n {
        if i % 2 == 0 {
            *map.entry(m - c[i]).or_insert(0) -= m;
        } else {
            *map.entry(c[i] + 1).or_insert(0) += m;
        }
    }
    // eprintln!("{:?}", &c);
    // eprintln!("{:?}", &map);

    let mut c0 = 0; // c[0] に相当する数
    let mut cur = result;

    for (&c1, &diff) in &map {
        cur += diff;
        if a.len() % 2 > 0 {
            cur += c1 - c0;
        }
        result = result.min(cur);
        c0 = c1;
    }

    println!("{result}");
}
