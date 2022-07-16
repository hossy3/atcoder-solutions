// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashMap;

// 003 - Longest Circular Road（★4）
// https://atcoder.rujp/contests/typical90/tasks/typical90_c

fn init_scores(start: usize, h: &HashMap<usize, Vec<usize>>, scores: &mut [i32]) {
    for i in 0..(scores.len()) {
        scores[i] = -1;
    }
    scores[start] = 0;

    let mut stack = Vec::new();
    stack.push(start);

    while let Some(pos) = stack.pop() {
        if let Some(v) = h.get(&pos) {
            for &next in v {
                if scores[next] < 0 {
                    scores[next] = scores[pos] + 1;
                    stack.push(next);
                }
            }
        }
    }
}

fn find_score(scores: &[i32]) -> (usize, i32) {
    let mut score = -1;
    let mut index = 0usize;
    for i in 0..(scores.len()) {
        if score < scores[i] {
            score = scores[i];
            index = i;
        }
    }
    (index, score)
}

fn main() {
    input! {
        n: usize,
        v: [(usize, usize); n - 1],
    }

    let mut h: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..n {
        h.insert(i, Vec::new());
    }

    for pair in v {
        let i = (&pair.0 - 1) as usize;
        let j = (&pair.1 - 1) as usize;
        h.get_mut(&i).unwrap().push(j);
        h.get_mut(&j).unwrap().push(i);
    }

    let mut scores = vec![0i32; n];
    init_scores(0, &h, &mut scores);
    let (index, _) = find_score(&scores);

    init_scores(index, &h, &mut scores);
    let (_, score) = find_score(&scores);
    println!("{}", score + 1);
}
