use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut result = 0;
    for &c in &s {
        result = result * 26 + (c as u8 - b'A') as usize + 1;
    }
    println!("{}", result);
}
