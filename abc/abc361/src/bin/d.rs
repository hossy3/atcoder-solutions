use std::collections::{HashMap, VecDeque};

use proconio::{input, marker::Chars};

fn f(s: Vec<char>, t: Vec<char>) -> i64 {
    let n = s.len();
    let mut map = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((s, 0));
    while let Some((s, step)) = queue.pop_front() {
        if map.contains_key(&s) {
            continue;
        }
        if s == t {
            return step as i64;
        }
        let i = s.iter().position(|&c| c == '.').unwrap();
        for j in 0..(n - 1) {
            if j.abs_diff(i) <= 1 {
                continue;
            }
            let mut s0 = s.clone();
            s0.swap(i, j);
            s0.swap(i + 1, j + 1);
            if map.contains_key(&s0) {
                continue;
            }
            queue.push_back((s0, step + 1));
        }
        map.insert(s, step);
    }

    -1
}

fn main() {
    input! {
        _: usize,
        mut s: Chars,
        mut t: Chars,
    }
    s.push('.');
    s.push('.');
    t.push('.');
    t.push('.');
    let result = f(s, t);
    println!("{result}");
}
