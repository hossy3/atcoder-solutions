use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
        b: [Usize1; m],
    }

    for b in b {
        a[b] += 1;
        if b > 0 {
            a[b - 1] += 1;
        }
        if b < n - 1 {
            a[b + 1] += 1;
        }
    }
    println!("{}", a.iter().join(" "));
}
