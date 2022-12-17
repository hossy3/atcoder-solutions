use proconio::{input, marker::Chars};

fn f(s: &[char], t: &[char]) -> bool {
    for i in 0..s.len() {
        if s[i] == 'x' && t[i] == 'x' {
            return false;
        }
    }
    true
}

fn main() {
    input! {
        n: usize,
        _: usize,
        s: [Chars; n],
    }

    let mut count = 0;
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            if f(&s[i], &s[j]) {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
