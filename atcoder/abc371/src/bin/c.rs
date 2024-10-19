use std::collections::HashSet;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn f(uv: &[(usize, usize)], ab: &[(usize, usize)], a: &[Vec<usize>], conv: &[usize]) -> usize {
    let score = |i: usize, j: usize| a[i][j - i - 1];

    let mut s = HashSet::new();
    for &(u, v) in uv {
        let (u, v) = (conv[u], conv[v]);
        let (u, v) = (u.min(v), u.max(v));
        s.insert((u, v));
    }

    let mut result = 0;
    for &(a0, b0) in ab {
        if s.contains(&(a0, b0)) {
            s.remove(&(a0, b0));
        } else {
            result += score(a0, b0);
        }
    }

    for (u, v) in s {
        result += score(u, v);
    }

    result
}

fn main() {
    input! {
        n: usize,
        mg: usize,
        uv: [(Usize1, Usize1); mg],
        mh: usize,
        ab: [(Usize1, Usize1); mh],
    }

    let mut a = vec![];
    for i in 0..(n - 1) {
        input! {
            a0: [usize; n - 1 - i],
        }
        a.push(a0);
    }

    let mut result = usize::MAX;
    for conv in (0..n).permutations(n) {
        result = result.min(f(&uv, &ab, &a, &conv));
    }

    println!("{result}");
}
