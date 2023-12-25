use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }
    let mut uf = Dsu::new(n);
    for &(a, b) in &ab {
        uf.merge(a, b);
    }
    let mut v = vec![0usize; n];
    for i in 0..n {
        v[uf.leader(i)] += 1;
    }
    let result = *v.iter().max().unwrap();
    println!("{result}");
}
