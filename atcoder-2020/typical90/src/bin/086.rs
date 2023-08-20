use proconio::{input, marker::Usize1};

const MOD: usize = 1_000_000_007;

fn f(i: usize, n: usize, xyzw: &[(usize, usize, usize, usize)]) -> usize {
    let b = 1 << i;
    let mut v = Vec::new();
    for &(x, y, z, w) in xyzw {
        v.push(((1 << x) | (1 << y) | (1 << z), (w & b) > 0));
    }

    let mut count: usize = 0;
    for j in 0..(1 << n) {
        if *(&v.iter().all(|&(vj, vw)| ((vj & j) > 0) == vw)) {
            count += 1;
        }
    }
    count
}

fn main() {
    input! {
        n: usize,
        xyzw: [(Usize1, Usize1, Usize1, usize)],
    }
    let mut count = 1;
    for i in 0..60 {
        count *= f(i, n, &xyzw);
        count %= MOD;
    }
    println!("{}", count);
}
