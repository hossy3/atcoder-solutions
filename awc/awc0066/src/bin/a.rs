use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        b: [Usize1; q],
    }

    for i in b {
        if i < n - 1 {
            a[i + 1] += a[i];
        }
        a[i] = 0;
    }

    println!("{}", a.iter().join(" "));
}
