use ac_library::Dsu;
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        uvw: [(Usize1, Usize1, usize); m],
    }

    let mut result = k;
    'outer: for v in (0..m).combinations(n - 1) {
        let mut uf = Dsu::new(n);
        let mut sum = 0;
        for &i in &v {
            let (u, v, w) = uvw[i];
            if uf.same(u, v) {
                continue 'outer;
            }
            uf.merge(u, v);
            sum += w;
        }
        result = result.min(sum % k);
    }

    println!("{result}");
}
