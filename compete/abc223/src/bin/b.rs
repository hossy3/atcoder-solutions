use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let mut s = VecDeque::from(s); 
    let mut s1 = s.clone();
    let mut s2 = s.clone();
    let len = s.len();
    for _ in 1..len {
        let c = s.pop_back().unwrap();
        s.push_front(c);
        if s1 > s {
            s1 = s.clone();
        }
        if s2 < s {
            s2 = s.clone();
        }
    }
    println!("{}", s1.iter().join(""));
    println!("{}", s2.iter().join(""));
}
