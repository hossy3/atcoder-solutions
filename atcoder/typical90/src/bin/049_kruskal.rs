use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut clr: [(i64, Usize1, Usize1); m],
    }

    let mut result = 0i64;
    let mut uf = Dsu::new(n + 1);
    clr.sort();
    for &(c, l, r) in clr.iter() {
        if !uf.same(l, r + 1) {
            uf.merge(l, r + 1);
            result += c;
        }
    }

    if (0..n).any(|i| !uf.same(i, n)) {
        result = -1;
    }
    println!("{result}");
}
