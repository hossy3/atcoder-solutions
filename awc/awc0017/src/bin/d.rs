use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [isize; n],
        uvb: [(Usize1, Usize1, isize); m],
    }

    let mut mat = vec![vec![0isize; n]; n];
    for (u, v, b) in uvb {
        mat[u][v] = b;
    }

    let mut result = isize::MIN;
    for v in (0..n).combinations(k) {
        let mut result0: isize = v.iter().map(|&i| a[i]).sum();
        for i in 0..(k - 1) {
            for j in (i + 1)..k {
                let (u, v) = (v[i], v[j]);
                result0 -= mat[u][v];
            }
        }
        result = result.max(result0);
    }

    println!("{result}");
}
