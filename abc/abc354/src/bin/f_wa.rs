use itertools::Itertools;
use proconio::input;

/// longest increasing subsequence
fn lis(a: &[usize]) -> Vec<usize> {
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
            a: [usize; n],
        }

        let v = lis(&a);
        eprintln!("{:?}", &v);
        let mut bs = vec![false; n];
        let mut x0 = 0; // 前回の index + 1
        for (i, &x) in v.iter().enumerate() {
            bs[x] = true;
            if i == v.len() - 1 {
                break;
            }
            for j in x0..x {
                if a[j] >= a[x] && a[j] < a[v[i + 1]] {
                    bs[j] = true;
                }
            }
            x0 = x + 1;
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
