use itertools::Itertools;
use proconio::{input, marker::Chars};

fn f(s: Vec<char>) -> Vec<char> {
    let n = s.len();
    for i in 0..(n - 1) {
        if s[i] <= s[i + 1] {
            continue;
        }
        for j in (i + 1)..n {
            if s[j] <= s[i] {
                continue;
            }
            let mut result = s[..i].iter().cloned().collect_vec();
            result.append(&mut s[(i + 1)..j].iter().cloned().collect_vec().clone());
            result.push(s[i]);
            result.append(&mut s[j..].iter().cloned().collect_vec().clone());
            return result;
        }
        let mut result = s[..i].iter().cloned().collect_vec();
        result.append(&mut s[(i + 1)..].iter().cloned().collect_vec().clone());
        result.push(s[i]);
        return result;
    }

    s
}

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            _n: usize,
            s: Chars,
        }
        let s = f(s);
        println!("{}", s.iter().join(""));
    }
}
