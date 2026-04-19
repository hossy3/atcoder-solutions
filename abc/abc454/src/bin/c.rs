use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut graph = vec![HashSet::new(); n];
    for &(a, b) in &ab {
        graph[a].insert(b);
    }

    let mut set = HashSet::new();
    set.insert(0);
    let mut stack = vec![0];
    while let Some(x) = stack.pop() {
        for &x in &graph[x] {
            if set.insert(x) {
                stack.push(x);
            }
        }
    }

    let result = set.iter().count();
    println!("{result}");
}
