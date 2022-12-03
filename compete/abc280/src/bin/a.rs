use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        _: usize,
        s: [Chars; h],
    }
    let mut count = 0;
    for s in &s {
        for &c in s {
            if c == '#' {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
