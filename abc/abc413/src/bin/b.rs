use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    let mut set = HashSet::new();
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            let str = s[i].iter().chain(s[j].iter()).collect::<String>();
            set.insert(str);

            let str = s[j].iter().chain(s[i].iter()).collect::<String>();
            set.insert(str);
        }
    }

    let result = set.len();
    println!("{result}");
}
