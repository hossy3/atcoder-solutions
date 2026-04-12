use itertools::Itertools;
use proconio::{input, marker::Usize1};

const N: usize = 1_000_000_000;

fn main() {
    input! {
        n: Usize1,
    }

    let mut v = vec![]; // 2のべき乗
    let mut k = 1usize;
    while k < N {
        v.push(k);
        k *= 2;
    }

    let mut v0 = v.clone();
    for _ in 0..9 {
        let mut v1 = v0.clone();
        let n1 = v1.len();
        for i in 0..n1 {
            for &x in &v {
                let x = v1[i] + 10usize.pow(v1[i].ilog10() + 1) * x;
                if x > N {
                    break;
                }
                v1.push(x);
            }
        }
        v0 = v1.iter().sorted().unique().cloned().collect();
    }

    println!("{}", v0[n]);
}
