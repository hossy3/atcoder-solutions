use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        lr: [(usize, usize); q],
    }

    let mut acc = vec![0usize; n + 1];
    for i in 0..(n - 1) {
        acc[i + 1] = acc[i] + if s[i + 1] == s[i] { 1 } else { 0 };
    }
    for &(l, r) in &lr {
        let result = acc[r - 1] - acc[l - 1];
        println!("{result}");
    }
}
