use std::collections::BTreeSet;

use proconio::input;

fn build_grundy() -> Vec<usize> {
    const N: usize = 100_001;
    let mut grundy = vec![0; N]; // 0 は使わない

    for i in 2..N {
        let j_max = (N as f64).sqrt() as usize;
        let mut mex = BTreeSet::new();
        for j in 1..=j_max {
            if i % j > 0 {
                continue;
            }
            mex.insert(grundy[j]);
            mex.insert(grundy[i / j]);
        }
        grundy[i] = *mex.last().unwrap() + 1;
    }

    grundy
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let grundy = build_grundy();
    let xor = a.iter().fold(0, |acc, &a| acc ^ grundy[a]);
    let yes = xor != 0;
    println!("{}", if yes { "Anna" } else { "Bruno" });
}
