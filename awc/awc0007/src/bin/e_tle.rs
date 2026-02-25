use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Usize1,
        t: Usize1,
        p: [Usize1; m],
    }

    let dist = |p0: usize, p1: usize| {
        let (x0, y0) = (p0 / n, p0 % n);
        let (x1, y1) = (p1 / n, p1 % n);
        x1.abs_diff(x0) + y1.abs_diff(y0)
    };

    if m == 0 {
        let result = dist(s, t);
        println!("{result}");
        return;
    }

    let mut result = usize::MAX;
    for v in (0..m).permutations(m) {
        let sum = v.windows(2).map(|w| dist(p[w[0]], p[w[1]])).sum::<usize>()
            + dist(s, p[v[0]])
            + dist(p[v[m - 1]], t);
        result = result.min(sum);
    }
    println!("{result}");
}
