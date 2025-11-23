use itertools::Itertools;
use proconio::{input_interactive, marker::Chars};

fn main() {
    input_interactive! {
        n: usize,
    }

    let m = (n - 1).ilog2() + 1;
    println!("{m}");

    for k in 0..m {
        let mut v = vec![];
        for i in 0..n {
            if i & (1 << k) > 0 {
                v.push(i + 1);
            }
        }
        println!("{} {}", v.len(), v.iter().join(" "));
    }

    input_interactive! {
        s: Chars,
    }

    let mut result = 1usize;
    for (i, &c) in s.iter().enumerate() {
        if c == '1' {
            result += 1 << i;
        }
    }
    println!("{result}");
}
