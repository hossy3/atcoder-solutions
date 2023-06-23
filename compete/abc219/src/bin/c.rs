use itertools::Itertools;
use proconio::{input, marker::Chars};

fn f(c: char) -> usize {
    (c as u8 - 'a' as u8) as usize
}

fn main() {
    input! {
        x: Chars,
        n: usize,
        s: [Chars; n],
    }
    let mut a = [0; 26];
    for (i, &c) in x.iter().enumerate() {
        a[f(c)] = i;
    }
    let mut v = vec![];
    for s in &s {
        let x = s.iter().map(|&c| a[f(c)]).collect_vec();
        v.push((x, s));
    }
    v.sort();
    for (_, s) in &v {
        let result = s.iter().join("");
        println!("{}", result);
    }
}
