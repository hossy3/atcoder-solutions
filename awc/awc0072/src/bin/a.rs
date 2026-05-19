use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
        t: Chars,
    }

    let result = std::iter::zip(s.iter(), t.iter())
        .map(|(&s, &t)| {
            if s == t {
                s
            } else if s == '?' {
                t
            } else if t == '?' {
                s
            } else {
                '!'
            }
        })
        .collect::<Vec<_>>();
    let yes = result.contains(&'!');
    println!("{}", result.iter().join(""));
    println!("{}", if yes { "Yes" } else { "No" });
}
