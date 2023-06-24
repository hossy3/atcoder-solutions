use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        s: Chars,
    }
    let mut v = vec![vec![]];
    for &c in &s {
        let n = v.len() - 1;
        if c == '(' {
            v.push(vec![c]);
        } else if c == ')' {
            if v.len() > 1 {
                v.pop();
            } else {
                v[n].push(c);
            }
        } else {
            v[n].push(c);
        }
    }
    let result = v.iter().map(|x| x.iter().join("")).join("");
    println!("{}", result);
}
