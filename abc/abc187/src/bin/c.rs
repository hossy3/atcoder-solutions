use std::collections::HashSet;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    let mut set0 = HashSet::<&[char]>::new();
    let mut set1 = HashSet::<&[char]>::new(); // with prefix !
    for s in &s {
        let s0 = if s[0] == '!' { &s[1..] } else { &s[0..] };
        if s[0] == '!' {
            set1.insert(s0);
        } else {
            set0.insert(s0);
        }
        if set0.contains(&s0) && set1.contains(&s0) {
            println!("{}", s0.iter().join(""));
            return;
        }
    }
    println!("satisfiable");
}
