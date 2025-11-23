use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        s: Chars,
    }
    for &c in &s {
        print!("{}{}", c, c);
    }
    println!();
}
