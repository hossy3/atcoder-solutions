// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashSet;

fn score_impl(nodes: &Vec<Vec<usize>>, x: usize, k: usize, set: &mut HashSet<usize>) {
    set.insert(x);
    if k > 0 {
        for &i in &nodes[x] {
            score_impl(&nodes, i, k - 1, set);
        }
    }
}

fn score(nodes: &Vec<Vec<usize>>, x: usize, k: usize) -> usize {
    let mut set = HashSet::<usize>::new();
    score_impl(nodes, x, k, &mut set);
    let mut result: usize = 0;
    for i in &set {
        result += i;
    }
    result
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
        q: usize,
        xk: [(usize, usize); q],
    };
    let mut nodes = vec![Vec::<usize>::new(); n + 1];

    for pair in ab {
        nodes[pair.0].push(pair.1);
        nodes[pair.1].push(pair.0);
    }

    for pair in xk {
        let result = score(&nodes, pair.0, pair.1);
        println!("{}", result);
    }
}
