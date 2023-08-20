use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    for &c in s.iter().rev() {
        let c = match c {
            '6' => '9',
            '9' => '6',
            _ => c,
        };
        print!("{}", c);
    }
    println!();
}
