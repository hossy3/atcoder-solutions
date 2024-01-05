use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    for c in s {
        let c = if c == '0' { '1' } else { '0' };
        print!("{}", c);
    }
    println!();
}
