use im_rc::HashSet;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        abcd: [(Usize1, char, Usize1, char); m],
    }

    let mut uf = UnionFind::new(n * 2);
    for i in 0..n {
        uf.union(i * 2, i * 2 + 1); // R: i * 2, B: i * 2 + 1
    }
    let mut count = 0;
    for &(a, b, c, d) in &abcd {
        let x = a * 2 + if b == 'R' { 0 } else { 1 };
        let y = c * 2 + if d == 'R' { 0 } else { 1 };
        if uf.equiv(x, y) {
            count += 1;
        } else {
            uf.union(x, y);
        }
    }

    let mut s = HashSet::new();
    for i in 0..n {
        let x = uf.find(i * 2);
        s.insert(x);
    }
    let count_noloop = s.len() - count;
    println!("{} {}", count, count_noloop);
}
