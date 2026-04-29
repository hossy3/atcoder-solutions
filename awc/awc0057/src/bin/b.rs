use proconio::{input, marker::Chars};

fn f(s: &[char], t: &[char]) -> bool {
    if s.len() < t.len() {
        return false;
    }
    let mut j = 0usize;
    for &s in s {
        if t[j] == s {
            j += 1;
            if j == t.len() {
                return true;
            }
        }
    }
    false
}

fn main() {
    input! {
        n: usize,
    }

    let mut s = vec![];
    for _ in 0..n {
        input! {
            k: usize,
            s0: [Chars; k],
        }
        s.push(s0);
    }

    input! {
        q: usize,
        t: [Chars; q],
    }
    for t in t {
        let result = s.iter().filter(|&s0| s0.iter().any(|s0| f(s0, &t))).count();
        println!("{result}");
    }
}
