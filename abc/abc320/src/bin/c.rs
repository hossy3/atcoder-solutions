use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        m: usize,
        s1: Chars,
        s2: Chars,
        s3: Chars,
    }
    let mut result = usize::MAX;
    for v in (0..(m * 3)).permutations(3) {
        if s1[v[0] % m] == s2[v[1] % m] && s2[v[1] % m] == s3[v[2] % m] {
            result = result.min(*v.iter().max().unwrap());
        }
    }
    if result == usize::MAX {
        println!("-1");
    } else {
        println!("{result}");
    }
}
