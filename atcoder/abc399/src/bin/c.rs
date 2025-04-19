use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }

    let mut dsu = Dsu::new(n);
    let mut result = 0usize;
    for &(u, v) in &uv {
        if dsu.same(u, v) {
            result += 1;
        } else {
            dsu.merge(u, v);
        }
    }
    println!("{result}");
}
