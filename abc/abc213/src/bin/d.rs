use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    }
    let mut v = vec![vec![]; n];
    for &(a, b) in &ab {
        v[a].push(b);
        v[b].push(a);
    }
    for i in 0..n {
        v[i].sort_by(|a, b| b.cmp(a));
    }

    let mut results = vec![];
    let mut prev = vec![std::usize::MAX; n];
    prev[0] = 0;
    let mut queue = vec![0];
    'outer: while let Some(i) = queue.pop() {
        results.push(i + 1);
        while let Some(j) = v[i].pop() {
            if prev[j] == std::usize::MAX {
                prev[j] = i;
                queue.push(j);
                continue 'outer;
            }
        }
        if i != 0 {
            queue.push(prev[i]);
            continue 'outer;
        }
    }
    println!("{}", results.iter().join(" "));
}
