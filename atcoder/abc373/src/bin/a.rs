use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [Chars; 12],
    }
    let result = s.iter().enumerate().filter(|(i, s)| i + 1 == s.len()).count();
    println!("{result}");
}
