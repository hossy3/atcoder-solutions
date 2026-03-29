use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        k: usize,
        s: Chars,
        t: Chars,
    }

    let result = std::iter::zip(s.iter(), t.iter())
        .filter(|(a, b)| a != b)
        .count()
        .saturating_sub(k);
    println!("{result}");
}
