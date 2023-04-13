// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [i64; n],
    }

    let mut results = vec![-1i64; n];
    let mut v0 = Vec::<i64>::new();
    let mut v1 = Vec::<Vec<i64>>::new();
    for i in 0..n {
        let x = p[i];
        if v0.is_empty() || v0.last().unwrap() < &x {
            if k == 1 {
                results[(x - 1) as usize] = i as i64 + 1;
            } else {
                v0.push(x);
                v1.push(vec![x]);
            }
        } else {
            let j = v0.binary_search(&x).unwrap_err(); // don't appear same value
            v0[j] = x;
            v1[j].push(x);
            if v1[j].len() == k {
                for y in &v1[j] {
                    results[(y - 1) as usize] = i as i64 + 1;
                }
                v0.remove(j);
                v1.remove(j);
            }
        }
    }
    for result in results {
        println!("{}", result);
    }
}
