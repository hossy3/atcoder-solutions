use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut v = vec![(1usize, 1usize); n];
    for i in 0..(n - 1) {
        if s[i] == 'A' {
            v[i + 1].0 = v[i].0 + 1;
        }
    }
    for i in (0..(n - 1)).rev() {
        if s[i] == 'B' {
            v[i].1 = v[i + 1].1 + 1;
        }
    }
    let result: usize = v.iter().map(|&x| x.0.max(x.1)).sum();
    println!("{}", result);
}
