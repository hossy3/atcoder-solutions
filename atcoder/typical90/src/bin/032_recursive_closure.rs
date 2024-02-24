use proconio::{input, marker::Usize1};
use std::collections::HashSet;

fn f<F0>(a: &[Vec<usize>], v: Vec<usize>, init: usize, f0: &F0) -> usize
where
    F0: Fn(usize, &[usize]) -> usize,
{
    if v.len() == a.len() {
        f0(init, &v)
    } else {
        let mut result = init;
        for i in 0..(a.len()) {
            if v.iter().any(|&x| x == i) {
                continue;
            }
            let mut v = v.clone();
            v.push(i);
            result = f(a, v, result, f0);
        }
        result
    }
}

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
        m: usize,
        xy: [(Usize1, Usize1); m],
    }
    let mut s = HashSet::<(usize, usize)>::new();
    for (x, y) in xy {
        s.insert((x, y));
        s.insert((y, x));
    }
    let v = vec![];
    let min_score = f(&a, v, usize::MAX, &|acc, v| {
        if v.windows(2).any(|v| s.contains(&(v[0], v[1]))) {
            acc
        } else {
            let score = v.iter().enumerate().fold(0, |acc, (j, &i)| acc + a[i][j]);
            acc.min(score)
        }
    });
    if min_score == usize::MAX {
        println!("-1");
    } else {
        println!("{min_score}");
    }
}
