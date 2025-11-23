use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        d: usize,
        s: Chars,
    }
    let result = s.iter().filter(|&&c| c == '.').count() + d;
    println!("{result}");
}
