use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut g = vec![vec![]; n + m];
    for i in 0..n {
        input! {
            a: usize,
            s: [Usize1; a],
        }
        for &j in &s {
            let j = n + j;
            g[i].push(j);
            g[j].push(i);
        }
    }

    let mut v = vec![false; n + m];
    let mut queue = VecDeque::new();
    queue.push_back((0, n));
    v[n] = true;
    while let Some((step, i)) = queue.pop_front() {
        for &j in &g[i] {
            if v[j] {
                continue;
            }
            if j == n + m - 1 {
                let result = step / 2;
                println!("{}", result);
                return;
            }
            queue.push_back((step + 1, j));
            v[j] = true;
        }
    }

    println!("{}", -1);
}
