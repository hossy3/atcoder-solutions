use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    for i in 0..h {
        let mut s = s[i].clone();
        for i in 0..(w - 1) {
            if s[i] == 'T' && s[i + 1] == 'T' {
                s[i] = 'P';
                s[i + 1] = 'C';
            }
        }
        let result = s.iter().join("");
        println!("{}", result);
    }
}
