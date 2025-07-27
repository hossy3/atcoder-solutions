use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut results = vec![];
    for (i, &c) in s.iter().enumerate() {
        if c == '#' {
            results.push(i + 1);
        }
    }
    for i in 0..(results.len() / 2) {
        println!("{},{}", results[i * 2], results[i * 2 + 1]);
    }
}
