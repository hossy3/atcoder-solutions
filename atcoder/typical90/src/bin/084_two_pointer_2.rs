use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut count = n * (n + 1) / 2;
    let mut l = 0usize;
    for r in 0..n {
        while s[l] != s[r] {
            l += 1;
        }
        count -= r + 1 - l;
    }
    println!("{count}");
}
