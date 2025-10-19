use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }
    let mut queue = vec![];
    let mut v = vec![vec![]; n]; // 次に候補になるもの
    for (i, &(a, b)) in ab.iter().enumerate() {
        if a == 0 && b == 0 {
            queue.push(i);
        } else if a == b {
            v[a - 1].push(i);
        } else {
            v[a - 1].push(i);
            v[b - 1].push(i);
        }
    }

    let mut set = HashSet::new();
    while let Some(i) = queue.pop() {
        if set.insert(i) {
            for &x in &v[i] {
                queue.push(x);
            }
            v[i].clear();
        }
    }

    let result = set.len();
    println!("{result}");
}
