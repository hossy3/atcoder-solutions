use itertools::Itertools;
use proconio::input;

/// longest increasing subsequence
fn lis(a: &[i64]) -> Vec<usize> {
    let mut v = vec![0; a.len()];
    let mut p = vec![];

    for (i, &x) in a.iter().enumerate() {
        let j = p.partition_point(|&y| y < x);
        if j == p.len() {
            p.push(x);
        } else if p[j] > x {
            p[j] = x;
        }
        v[i] = j + 1;
    }
    v
}

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            a: [i64; n],
        }

        let v = lis(&a);
        let x_max = *v.iter().max().unwrap();

        let a0: Vec<i64> = a.iter().rev().map(|x| -x).collect();
        let v0 = lis(&a0);

        let mut bs = vec![false; n];
        for i in 0..n {
            if v[i] + v0[n - i - 1] == x_max + 1 {
                bs[i] = true;
            }
        }

        let mut results = vec![];
        for (i, &b) in bs.iter().enumerate() {
            if b {
                results.push(i + 1);
            }
        }

        println!("{}", results.len());
        println!("{}", results.iter().join(" "));
    }
}
