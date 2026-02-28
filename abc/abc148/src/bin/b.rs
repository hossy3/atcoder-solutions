use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
        t: Chars,
    }
    for (s, t) in std::iter::zip(s, t) {
        print!("{s}{t}");
    }
    println!();
}
