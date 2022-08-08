use std::collections::HashSet;

use proconio::input;

const MAX: usize = 1 << 60;

fn f(j: usize, v: &mut [usize], a: &[Vec<usize>], s: &HashSet<(usize, usize)>, score: usize) -> usize {
    let mut min_score = MAX;
    for i in 0..v.len() {
        if v[..j].iter().any(|&x| x == i) {
            continue;
        }
        if j > 0 && s.contains(&(v[j - 1], i)) {
            continue;
        }
        v[j] = i;
        let score = score + a[i][j];
        if j + 1 == v.len() {
            min_score = min_score.min(score);
        } else {
            min_score = min_score.min(f(j + 1, v, a, s, score));
        }
    }
    min_score
}

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
        m: usize,
        xy: [(usize, usize); m],
    }
    let mut s = HashSet::<(usize, usize)>::new();
    for (x, y) in xy {
        s.insert((x - 1, y - 1));
        s.insert((y - 1, x - 1));
    }
    let mut v = vec![0; n];
    let min_score = f(0, &mut v, &a, &s, 0);
    if min_score == MAX {
        println!("-1");
    } else {
        println!("{}", min_score);
    }
}
