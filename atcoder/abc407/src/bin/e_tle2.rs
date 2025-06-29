use std::collections::VecDeque;

use proconio::input;

fn f(a: &[usize]) -> usize {
    let n = a.len() / 2;
    let mut cur = VecDeque::new();
    cur.push_back(0usize);
    for (i, &x) in a[0..n].iter().enumerate() {
        cur.push_back(0);
        for j in (0..(cur.len() - 1)).rev() {
            cur[j + 1] = cur[j + 1].max(cur[j] + x);
        }
        if i % 2 == 0 {
            cur.pop_front();
        }
    }

    for (i, &x) in a[n..].iter().enumerate() {
        let i = i + n;
        for j in (0..(cur.len() - 1)).rev() {
            cur[j + 1] = cur[j + 1].max(cur[j] + x);
        }
        if i % 2 == 0 {
            cur.pop_front();
        }
    }

    cur[0]
}

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            a: [usize; n * 2],
        }
        let result = f(&a);
        println!("{result}");
    }
}
