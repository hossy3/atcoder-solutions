use itertools::Itertools;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        t: Chars,
        lr: [(Usize1, Usize1); m],
    }
    let mut cum = vec![0_i64; n + 1];
    for (l, r) in lr {
        cum[l] += 1;
        cum[r + 1] -= 1;
    }
    let mut a = vec![0_i64; n + 1];
    for i in 0..n {
        a[i + 1] = a[i] + cum[i];
    }
    let result = (0..n)
        .map(|i| if a[i + 1] % 2 == 0 { s[i] } else { t[i] })
        .join("");
    println!("{result}");
}
