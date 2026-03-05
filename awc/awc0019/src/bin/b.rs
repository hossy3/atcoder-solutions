use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        s: [Chars; n],
    }
    let result = s
        .iter()
        .filter(|s| s.iter().filter(|&&c| c == '!').count() >= k)
        .count();
    println!("{result}");
}
