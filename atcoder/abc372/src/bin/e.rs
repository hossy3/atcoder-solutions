use std::cmp::Reverse;

use ac_library::Dsu;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        query: [(u8, usize, usize); q],
    }
    let mut m = vec![vec![]; n];
    for i in 0..n {
        m[i].push(i);
    }
    let mut dsu = Dsu::new(n);
    for (k, x, y) in query {
        match k {
            1 => {
                let (u, v) = (x - 1, y - 1);
                let u0 = dsu.leader(u);
                let v0 = dsu.leader(v);
                if dsu.same(u0, v0) {
                    continue;
                }
                let w = dsu.merge(u, v);

                let mut vv = m[u0].clone();
                for &x in &m[v0] {
                    vv.push(x);
                }
                vv.sort_by_key(|&x| Reverse(x));
                while vv.len() > 10 {
                    vv.pop();
                }
 
                m[u0].clear();
                m[v0].clear();
                m[w] = vv;
            }
            2 => {
                let (u, k) = (dsu.leader(x - 1), y);
                if m[u].len() < k {
                    println!("-1");
                } else {
                    println!("{}", m[u][k - 1] + 1);
                }
            }
            _ => unreachable!(),
        }
    }
}
