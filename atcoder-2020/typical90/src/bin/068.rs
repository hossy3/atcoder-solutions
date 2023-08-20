use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn init_ambiguous(n: usize, txyv: &[(usize, usize, usize, i64)]) -> Vec<bool> {
    let mut q = vec![false; txyv.len()];
    let mut uf = UnionFind::new(n);
    for (i, &(t, x, y, _)) in txyv.iter().enumerate() {
        if t == 0 {
            uf.union(x, y);
        } else {
            q[i] = !uf.equiv(x, y);
        }
    }
    q
}

fn init_sample(n: usize, txyv: &[(usize, usize, usize, i64)]) -> Vec<(i64, i64)> {
    let mut a = vec![-1; n];
    for &(t, x, _, v) in txyv {
        if t == 0 {
            a[x] = v;
        }
    }

    let mut s = vec![(0, 0); n];
    for i in 0..n {
        if i == 0 || a[i - 1] == -1 {
            s[i] = (0, 1);
        } else {
            s[i] = (a[i - 1] - s[i - 1].0, a[i - 1] - s[i - 1].1);
        }
    }
    s
}

fn main() {
    input! {
        n: usize,
        txyv: [(usize, Usize1, Usize1, i64)],
    }

    let q = init_ambiguous(n, &txyv);
    let s = init_sample(n, &txyv);

    for (i, &(t, x, y, v)) in txyv.iter().enumerate() {
        if t == 1 {
            if q[i] {
                println!("Ambiguous");
            } else {
                let z = s[y].0 + (v - s[x].0) * (s[x].1 - s[x].0) * (s[y].1 - s[y].0);
                println!("{}", z);
            }
        }
    }
}
