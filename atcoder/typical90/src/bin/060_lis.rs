use proconio::input;
use std::iter;

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
        a: [usize],
    }
    let v0 = lis(&a);

    let mut a1 = a;
    a1.reverse();
    let mut v1 = lis(&a1);
    v1.reverse();

    let count = iter::zip(v0, v1).map(|(x0, x1)| x0 + x1 - 1).max().unwrap();
    println!("{count}");
}
