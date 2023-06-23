use proconio::{input, marker::Chars};

fn main() {
    input! {
        s1: String,
        s2: String,
        s3: String,
        t: Chars,
    }
    for &c in &t {
        let s = match c {
            '1' => &s1,
            '2' => &s2,
            _ => &s3,
        };
        print!("{}", s);
    }
    println!();
}
