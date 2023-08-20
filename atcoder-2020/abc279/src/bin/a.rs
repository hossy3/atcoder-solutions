use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut count = 0;
    for &c in &s {
        count += if c == 'v' { 1 } else { 2 };
    }
    println!("{}", count);
}
