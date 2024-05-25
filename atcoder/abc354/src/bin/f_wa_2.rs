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
        let mut bs0 = vec![false; n];
        let x_max = *v.iter().max().unwrap();
        let mut v0 = vec![usize::MAX; x_max + 1];
        for (i, &x) in v.iter().enumerate().rev() {
            if x == x_max || a[i] < v0[x + 1] {
                bs0[i] = true;
                v0[x] = v0[x].min(a[i]);
            }
        }

        let mut bs1 = vec![false; n];
        let x_max = *v.iter().max().unwrap();
        let mut v1 = vec![0; x_max + 1];
        for (i, &x) in v.iter().enumerate() {
            if x == 0 || a[i] > v1[x - 1] {
                bs1[i] = true;
                v1[x] = v1[x].max(a[i]);
            }
        }

        let mut results = vec![];
        for (i, &b) in bs0.iter().enumerate() {
            if b && bs1[i] {
                results.push(i + 1);
            }
        }

        println!("{}", results.len());
        println!("{}", results.iter().join(" "));
    }
}
