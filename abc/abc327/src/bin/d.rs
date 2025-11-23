use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Usize1; m],
        b: [Usize1; m],
    }
    let mut uf = Dsu::new(n * 2);
    for i in 0..m {
        uf.merge(a[i], b[i] + n);
        uf.merge(a[i] + n, b[i]);
    }
    let yes = (0..n).all(|i| !uf.same(i, i + n));
    println!("{}", if yes { "Yes" } else { "No" });
}
