use proconio::{input, marker::Usize1};
use superslice::Ext;

fn f(n: usize, xy: &[(usize, usize)], group: &[usize], (a, b): (usize, usize)) -> bool {
    if group[a] == group[b] {
        return true;
    }
    if a > group[b] {
        return false;
    }
    let b = group[b];

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

// 分岐なく直列につながっているグループを返す。グループ番号は先頭のノード番号とする
fn build_sequential_group(n: usize, xy: &[(usize, usize)]) -> Vec<usize> {
    let mut nedges = vec![(0usize, 0usize); n];
    for &(x, y) in xy {
        nedges[x].0 += 1;
        nedges[y].1 += 1;
    }

    let mut v: Vec<usize> = (0..n).collect();
    for &(x, y) in xy {
        if nedges[x].0 > 1 || nedges[x].1 > 1 || nedges[y].0 > 1 || nedges[y].1 > 1 {
            continue;
        }
        v[y] = v[x];
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(build_sequential_group(3, &[(0, 1), (1, 2)]), vec![0, 0, 0]);
        assert_eq!(
            build_sequential_group(5, &[(0, 1), (1, 2), (2, 3), (3, 4)]),
            vec![0, 0, 0, 0, 0]
        );
        assert_eq!(
            build_sequential_group(4, &[(0, 1), (1, 2), (1, 3)]),
            vec![0, 1, 2, 3]
        );
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

    let group = build_sequential_group(n, &xy);

    for (a, b) in ab {
        let yes = f(n, &xy, &group, (a, b));
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
