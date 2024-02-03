use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn f(n: usize, clr: &[(i64, usize, usize)]) -> i64 {
    let mut result = 0i64;
    let mut uf = Dsu::new(n + 1);
    let mut clr = Vec::from_iter(clr.iter());
    clr.sort();
    for &&(c, l, r) in clr.iter() {
        if !uf.same(l, r + 1) {
            uf.merge(l, r + 1);
            result += c;
        }
    }

    if (0..n).any(|i| !uf.same(i, n)) {
        result = -1;
    }
    result
}

fn main() {
    input! {
        n: usize,
        m: usize,
        clr: [(i64, Usize1, Usize1); m],
    }
    let result = f(n, &clr);
    println!("{result}");
}
