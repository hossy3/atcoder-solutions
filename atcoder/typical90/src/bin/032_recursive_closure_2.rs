use proconio::{input, marker::Usize1};
use std::collections::HashSet;

fn f_recur<Callback>(
    n: usize,
    s: &HashSet<(usize, usize)>,
    v: &mut Vec<usize>,
    used: &mut Vec<bool>,
    init: usize,
    cb: &Callback,
) -> usize
where
    Callback: Fn(usize, &[usize]) -> usize,
{
    if v.len() == n {
        cb(init, &v)
    } else {
        let mut result = init;
        for i in 0..n {
            if used[i] {
                continue;
            }
            if let Some(&j) = v.last() {
                if s.contains(&(i, j)) {
                    continue;
                }
            }

            used[i] = true;
            v.push(i);
            result = f_recur(n, s, v, used, result, cb);
            v.pop();
            used[i] = false;
        }
        result
    }
}

fn f<Callback>(n: usize, s: &HashSet<(usize, usize)>, cb: Callback) -> usize
where
    Callback: Fn(usize, &[usize]) -> usize,
{
    let mut v = vec![];
    let mut used = vec![false; n];
    let init = usize::MAX;
    f_recur(n, s, &mut v, &mut used, init, &cb)
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
    let min_score = f(n, &s, |acc, v| {
        let score = v.iter().enumerate().fold(0, |acc, (j, &i)| acc + a[i][j]);
        acc.min(score)
    });
    if min_score == usize::MAX {
        println!("-1");
    } else {
        println!("{min_score}");
    }
}
