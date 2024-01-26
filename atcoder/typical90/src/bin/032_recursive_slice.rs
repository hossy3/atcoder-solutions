use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn f(
    a: &[Vec<usize>],
    s: &HashSet<(usize, usize)>,
    v: [usize; 10],
    j: usize,
    score: usize,
) -> usize {
    let n = a.len();
    let mut min_score = usize::MAX;
    for i in 0..n {
        if v[..j].iter().any(|&x| x == i) {
            continue;
        }
        if j > 0 && s.contains(&(v[j - 1], i)) {
            continue;
        }
        let score = score + a[i][j];
        if j + 1 == n {
            min_score = min_score.min(score);
        } else {
            let mut v = v;
            v[j] = i;
            min_score = min_score.min(f(a, s, v, j + 1, score));
        }
    }
    min_score
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
    let v = [0; 10];
    let min_score = f(&a, &s, v, 0, 0);
    if min_score == usize::MAX {
        println!("-1");
    } else {
        println!("{min_score}");
    }
}
