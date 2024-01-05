use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn f(s: &[char], k: usize) -> usize {
    let mut result = 0;
    let mut l = 0;
    let mut v = VecDeque::new();
    for (r, &c) in s.iter().enumerate() {
        if c == 'X' {
            result = result.max(r - l + 1);
        } else if v.len() < k {
            v.push_back(r);
            result = result.max(r - l + 1);
        } else {
            v.push_back(r);
            l = v.pop_front().unwrap() + 1;
        }
    }
    result
}

#[test]
fn test_func() {
    use itertools::Itertools;
    assert_eq!(f(&"XX...X.X.X.".chars().collect_vec(), 2), 5);
    assert_eq!(f(&"XX...X.X.X.".chars().collect_vec(), 0), 2);
    assert_eq!(f(&"XXXX".chars().collect_vec(), 200000), 4);
}

fn main() {
    input! {
        s: Chars,
        k: usize,
    }
    let result = f(&s, k);
    println!("{}", result);
}
