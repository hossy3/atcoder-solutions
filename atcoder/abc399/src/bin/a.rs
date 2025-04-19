use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
        t: Chars,
    }

    let result = std::iter::zip(s.iter(), t.iter())
        .filter(|(&s, &t)| s != t)
        .count();
    println!("{result}");
}
