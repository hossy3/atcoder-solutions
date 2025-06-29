use std::collections::HashSet;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn f(ab: &[(usize, usize)], v: &[usize]) -> usize {
    let n = v.len();
    let mut result = 0;
    let x = v[0];
    for v in v[1..].iter().cloned().permutations(n - 1) {
        let mut set = HashSet::new();
        for v in v.windows(2) {
            set.insert((v[0], v[1]));
            set.insert((v[1], v[0]));
        }
        set.insert((x, v[0]));
        set.insert((v[0], x));
        set.insert((x, v[n - 2]));
        set.insert((v[n - 2], x));

        let mut count = 0usize;
        for &(a, b) in ab {
            if set.contains(&(a, b)) {
                set.remove(&(a, b));
                set.remove(&(b, a));
                count += 1; // いくつ使ったかを返す
            }
        }
        result = result.max(count);
    }

    result
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let v: Vec<_> = (0..n).collect();
    let x = f(&ab, &v);
    let mut result = (n - x) + (ab.len() - x);
    if n == 6 {
        let mut set = HashSet::new();
        for vv in [0, 0, 0, 1, 1, 1].iter().permutations(n) {
            if !set.insert(vv.clone()) {
                continue;
            }
            let v0: Vec<_> = (0..n).filter(|&i| *vv[i] == 0).collect();
            let v1: Vec<_> = (0..n).filter(|&i| *vv[i] == 1).collect();
            let x0 = f(&ab, &v0);
            let x1 = f(&ab, &v1);
            let x = x0 + x1;
            result = result.min((n - x) + (ab.len() - x));
        }
    }
    if n == 7 {
        let mut set = HashSet::new();
        for vv in [0, 0, 0, 0, 1, 1, 1].iter().permutations(n) {
            if !set.insert(vv.clone()) {
                continue;
            }
            let v0: Vec<_> = (0..n).filter(|&i| *vv[i] == 0).collect();
            let v1: Vec<_> = (0..n).filter(|&i| *vv[i] == 1).collect();
            let x0 = f(&ab, &v0);
            let x1 = f(&ab, &v1);
            let x = x0 + x1;
            result = result.min((n - x) + (ab.len() - x));
        }
    }
    if n == 8 {
        let mut set = HashSet::new();
        for vv in [0, 0, 0, 0, 0, 1, 1, 1].iter().permutations(n) {
            if !set.insert(vv.clone()) {
                continue;
            }
            let v0: Vec<_> = (0..n).filter(|&i| *vv[i] == 0).collect();
            let v1: Vec<_> = (0..n).filter(|&i| *vv[i] == 1).collect();
            let x0 = f(&ab, &v0);
            let x1 = f(&ab, &v1);
            let x = x0 + x1;
            result = result.min((n - x) + (ab.len() - x));
        }

        let mut set = HashSet::new();
        for vv in [0, 0, 0, 0, 1, 1, 1, 1].iter().permutations(n) {
            if !set.insert(vv.clone()) {
                continue;
            }
            let v0: Vec<_> = (0..n).filter(|&i| *vv[i] == 0).collect();
            let v1: Vec<_> = (0..n).filter(|&i| *vv[i] == 1).collect();
            let x0 = f(&ab, &v0);
            let x1 = f(&ab, &v1);
            let x = x0 + x1;
            result = result.min((n - x) + (ab.len() - x));
        }
    }
    println!("{result}");
}
