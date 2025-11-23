use proconio::{input, marker::Usize1};

fn f(n: usize, a: &[usize]) -> usize {
    let mut v = vec![false; n];
    for &x in a {
        v[x] = !v[x];
    }

    let mut v0 = vec![];
    for (i, &x) in v.iter().enumerate() {
        if x {
            v0.push(i + 1);
        }
    }
    if v0.len() < 2 {
        return 0;
    }

    let mut v1 = vec![0usize];
    let mut v2 = vec![0usize];
    for (i, v3) in v0.windows(2).enumerate() {
        if i % 2 == 0 {
            v1.push(v3[1] - v3[0]);
        } else {
            v2.push(v3[1] - v3[0]);
        }
    }

    for i in 1..v1.len() {
        v1[i] += v1[i - 1];
    }
    for i in 1..v2.len() {
        v2[i] += v2[i - 1];
    }
    if v0.len() % 2 == 0 {
        return v1[v1.len() - 1];
    }

    let mut result = usize::MAX;
    for i in 0..v1.len() {
        result = result.min(v1[i] + v2[v2.len() - 1] - v2[i]);
    }
    result
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [Usize1; k],
    }
    let result = f(n, &a);
    println!("{result}");
}
