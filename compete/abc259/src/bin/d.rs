// -*- coding:utf-8-unix -*-

use petgraph::unionfind::UnionFind;
use proconio::input;

fn dist2(x0: i64, y0: i64, x1: i64, y1: i64) -> i64 {
    (x1 - x0).pow(2) + (y1 - y0).pow(2)
}

fn main() {
    input! {
        n: usize,
        sxy: (i64, i64),
        txy: (i64, i64),
        xyrs: [(i64, i64, i64); n],
    }

    let mut uf = UnionFind::new(n);
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            let d2 = dist2(xyrs[i].0, xyrs[i].1, xyrs[j].0, xyrs[j].1);
            if (xyrs[i].2 + xyrs[j].2).pow(2) >= d2 && (xyrs[i].2 - xyrs[j].2).pow(2) <= d2 {
                uf.union(i, j);
            }
        }
    }

    let s0 = xyrs.iter().position(|&xyr| (xyr.2).pow(2) == dist2(sxy.0, sxy.1, xyr.0, xyr.1)).unwrap();
    let t0 = xyrs.iter().position(|&xyr| (xyr.2).pow(2) == dist2(txy.0, txy.1, xyr.0, xyr.1)).unwrap();
    let yes = uf.find(s0) == uf.find(t0);
    println!("{}", if yes { "Yes" } else { "No" });
}
