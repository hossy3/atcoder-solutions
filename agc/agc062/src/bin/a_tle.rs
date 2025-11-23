use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            mut s: Chars,
        }
        for i in 0..(n - 1) {
            let result = s.iter().join("");
            eprintln!("{}", result);
            let mut s0 = Vec::with_capacity(n - 1); // A
            let mut s1 = Vec::with_capacity(n - 1); // B
            for j in 0..(n - i - 1) {
                if s[j] == 'A' {
                    s0.push(s[j + 1]);
                } else {
                    s1.push(s[j + 1]);
                }
            }
            s0.append(&mut s1);
            s = s0;
        }
        let result = s.iter().join("");
        println!("{}", result);
    }
}
