use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn f(n: usize, m: usize, s: &[char], c: &[usize]) -> Vec<char> {
    let mut v = vec![VecDeque::new(); m];
    for (i, &x) in c.iter().enumerate() {
        v[x].push_back(s[i]);
    }
    for x in 0..m {
        if let Some(y) = v[x].pop_back() {
            v[x].push_front(y);
        }
    }
    let mut results = Vec::with_capacity(n);
    for &x in c {
        if let Some(y) = v[x].pop_front() {
            results.push(y);
        }
    }
    results
}

#[test]
fn test_func() {
    assert_eq!(
        f(
            8,
            3,
            &"apzbqrcs".chars().collect_vec(),
            &[0, 1, 2, 0, 1, 1, 0, 1]
        ),
        "cszapqbr".chars().collect_vec()
    );

    assert_eq!(
        f(2, 1, &"aa".chars().collect_vec(), &[0, 0]),
        "aa".chars().collect_vec()
    );
}

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        c: [Usize1; n],
    }
    let results = f(n, m, &s, &c);
    println!("{}", results.iter().join(""));
}
