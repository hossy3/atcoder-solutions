use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        uvw: [(Usize1, Usize1, u32); m],
    }

    let mut result = 0u32;
    let mut no_use = 0u32;
    for k in (0..30).rev() {
        let k0 = 1 << k;
        let mut dsu = Dsu::new(n);
        for &(u, v, w) in &uvw {
            if w & (no_use | k0) == 0 {
                dsu.merge(u, v);
            }
        }

        if dsu.same(0, n - 1) {
            no_use |= k0; // これがなくてもひとつなぎにできる
        } else {
            result |= k0;
        }
    }

    println!("{result}");
}
