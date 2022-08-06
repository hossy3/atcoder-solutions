use petgraph::unionfind::UnionFind;
use proconio::input;

fn f(w: usize, r: usize, c: usize) -> usize {
    (r - 1) * w + (c - 1)
}
fn main() {
    input! {
        h: usize,
        w: usize,
        qn: usize,
    }
    let mut uf = UnionFind::new(h * w);
    let mut m = vec![vec![false; w + 2]; h + 2];
    for _ in 0..qn {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                r: usize,
                c: usize,
            }
            m[r][c] = true;
            if m[r - 1][c] {
                uf.union(f(w, r, c), f(w, r - 1, c));
            }
            if m[r + 1][c] {
                uf.union(f(w, r, c), f(w, r + 1, c));
            }
            if m[r][c - 1] {
                uf.union(f(w, r, c), f(w, r, c - 1));
            }
            if m[r][c + 1] {
                uf.union(f(w, r, c), f(w, r, c + 1));
            }
        } else if t == 2 {
            input! {
                ra: usize,
                ca: usize,
                rb: usize,
                cb: usize,
            }
            let yes = m[ra][ca] && m[rb][cb] && uf.equiv(f(w, ra, ca), f(w, rb, cb));
            println!("{}", if yes { "Yes" } else { "No" });
        }
    }
}
