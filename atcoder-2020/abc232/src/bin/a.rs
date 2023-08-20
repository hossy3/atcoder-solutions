use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let result = (s[0] as usize - '0' as usize) * (s[2] as usize - '0' as usize);
    println!("{}", result);
}
