use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        w: Chars,
    }
    let result = std::iter::zip(s, w).filter(|t| t.0 != t.1).count();
    println!("{result}");
}
