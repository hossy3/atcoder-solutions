use std::collections::HashMap;

use proconio::input;

fn f(a: &[usize]) -> usize {
    let n = a.len();

    // 3, 7 の累積和
    let mut cum3 = vec![0usize; n + 1];
    for (i, &x) in a.iter().enumerate() {
        cum3[i + 1] = cum3[i];
        if x == 3 {
            cum3[i + 1] += 1;
        };
    }

    let mut cum7 = vec![0usize; n + 1];
    for (i, &x) in a.iter().enumerate() {
        cum7[i + 1] = cum7[i];
        if x == 7 {
            cum7[i + 1] += 1;
        };
    }

    let mut result = 0usize;
    for (i, &x) in a.iter().enumerate() {
        if x == 5 {
            result += cum3[i + 1] * cum7[i + 1];
            result += (cum3[n] - cum3[i + 1]) * (cum7[n] - cum7[i + 1]);
        };
    }

    result
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    // 倍数ごとに分ける
    let mut map = HashMap::new();
    for &x in &a {
        if x % 3 == 0 {
            map.entry(x / 3).or_insert(vec![]).push(3);
        }
        if x % 5 == 0 {
            map.entry(x / 5).or_insert(vec![]).push(5);
        }
        if x % 7 == 0 {
            map.entry(x / 7).or_insert(vec![]).push(7);
        }
    }

    let mut result = 0;
    for (_, v) in map {
        result += f(&v);
    }
    println!("{result}");
}
