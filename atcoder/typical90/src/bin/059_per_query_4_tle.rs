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

fn is_all_linear(n: usize, xy: &[(usize, usize)]) -> bool {
    let m = xy.len();
    xy.windows(2).all(|v| {
        let ((x0, _), (x1, y1)) = (v[0], v[1]);
        (x0 == x1) || (x0 + 1 == x1 && x1 + 1 == y1)
    }) && xy[0] == (0, 1)
        && xy[m - 1] == (n - 2, n - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(is_all_linear(4, &[(0, 1), (1, 2), (2, 3)]), true);
        assert_eq!(is_all_linear(4, &[(0, 1), (0, 2), (1, 2), (2, 3)]), true);
        assert_eq!(is_all_linear(4, &[(0, 1), (0, 3), (1, 3), (2, 3)]), false);
    }
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
    let all_linear = is_all_linear(n, &xy);
    for (a, b) in ab {
        let yes = all_linear || f(n, &xy, (a, b));
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
