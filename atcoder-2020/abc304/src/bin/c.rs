use petgraph::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {
        n: usize,
        d: i64,
        xy: [(i64, i64); n],
    }

    let mut uf = UnionFind::new(n);
    let d2 = d.pow(2);
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            let (x0, y0) = xy[i];
            let (x1, y1) = xy[j];
            if (x0 - x1).pow(2) + (y0 - y1).pow(2) <= d2 {
                uf.union(i, j);
            }
        }
    }

    for i in 0..n {
        let yes = uf.equiv(0, i);
        println!("{}", if yes { "Yes" } else { "No" });
    }
}
