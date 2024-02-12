use proconio::{input, marker::Usize1};
use superslice::Ext;

fn f(n: usize, xy: &[(usize, usize)], (a, b): (usize, usize)) -> bool {
    let i0 = xy.lower_bound_by_key(&a, |&(x, _)| x);
    if i0 == xy.len() || xy[i0].0 != a {
        return false;
    }
    let i1 = xy.lower_bound_by_key(&b, |&(x, _)| x);

    let mut v = vec![false; n];
    v[a] = true;

    for &(x, y) in &xy[i0..i1] {
        if v[x] {
            v[y] = true;
        }
    }
    v[b]
}

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        mut xy: [(Usize1, Usize1); m],
        ab: [(Usize1, Usize1); q],
    }
    xy.sort();

    let all_linear = xy.windows(2).all(|v| v[1].0 - v[0].0 <= 1)
        && xy.iter().all(|&(x, y)| x + 1 == y)
        && xy[0].0 == 0
        && xy[m - 1].1 == n - 1;

    for (a, b) in ab {
        let yes = all_linear || f(n, &xy, (a, b));
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
