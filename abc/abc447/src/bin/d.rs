use std::collections::BTreeSet;

use proconio::{input, marker::Chars};

fn f(s: &[char]) -> usize {
    let mut a = BTreeSet::new();
    let mut b = BTreeSet::new();
    let mut c = BTreeSet::new();
    for (i, &s0) in s.iter().enumerate() {
        match s0 {
            'A' => {
                a.insert(i);
            }
            'B' => {
                b.insert(i);
            }
            'C' => {
                c.insert(i);
            }
            _ => unreachable!(),
        }
    }

    let mut result = 0;
    while let Some(a0) = a.pop_first() {
        let Some(&b0) = b.range(a0..).next() else {
            break;
        };
        let Some(&c0) = c.range(b0..).next() else {
            break;
        };
        result += 1;
        b.remove(&b0);
        c.remove(&c0);
    }
    result
}

fn main() {
    input! {
        s: Chars,
    }
    let result = f(&s);
    println!("{result}");
}
