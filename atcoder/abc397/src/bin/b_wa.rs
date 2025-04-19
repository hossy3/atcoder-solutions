use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let result =
        s.windows(2).filter(|v| v[0] == v[1]).count() + if s.last() == Some(&'i') { 1 } else { 0 };
    println!("{result}");
}
