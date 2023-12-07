use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        s: Chars,
    }
    let mut best_score = 1_000_000_000_000_000_usize;
    for i in 0..n {
        let mut score = a * i;
        for j in 0..(n / 2) {
            let i0 = (i + j) % n;
            let i1 = (i + n - 1 - j) % n;
            if s[i0] != s[i1] {
                score += b;
            }
        }
        best_score = best_score.min(score);
    }
    println!("{}", best_score);
}
