use std::collections::BTreeMap;

use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn f(v: &[usize], t: usize, rest: usize) -> usize {
    if v.len() == 0 {
        return if rest == 0 { 1 } else { 0 };
    }
    let mut x = 0;
    for i in 0..(rest.min(v[0])) {
        let mut x0 = 1;
        for i0 in 0..(v[0] - i) {
            x0 *= t - rest - i0;
        }
        for i0 in 0..i {
            x0 *= rest - i0;
        }
        x += x0 * f(&v[1..], t, rest - i); 
    }
    x
}

fn main() {
    input! {
        n: usize,
        t: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut uf = UnionFind::new(n);
    for &(a, b) in &ab {
        uf.union(a, b);
    }

    let mut map = BTreeMap::new();
    for i in 0..n {
        *map.entry(uf.find(i)).or_insert(0) += 1;
    }
    let mut v = vec![];
    for (_, &x) in &map {
        v.push(x);
    }
    v.sort_by(|a, b| b.cmp(a));
    if v[0] > t {
        println!("0");
        return;
    }

    let mul = (1..=n).fold(1usize, |acc, x| acc * x);
    let result = f(&v, t, t) / mul;
    
    println!("{}", result);
}
