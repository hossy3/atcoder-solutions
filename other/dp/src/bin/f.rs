use itertools::Itertools;
use proconio::{input, marker::Chars};

/// 二次元DP
fn solve_dp(s: &[char], t: &[char]) -> Vec<char> {
    let mut state = vec![vec![(0, (s.len(), t.len())); t.len() + 1]; s.len() + 1];
    for (i, &s) in s.iter().enumerate() {
        for (j, &t) in t.iter().enumerate() {
            if s == t {
                state[i + 1][j + 1] = (state[i][j].0 + 1, (i, j));
            } else {
                if state[i + 1][j].0 > state[i][j + 1].0 {
                    state[i + 1][j + 1] = state[i + 1][j];
                } else {
                    state[i + 1][j + 1] = state[i][j + 1];
                }
            }
        }
    }

    // 答えを復元する
    let mut chars = vec![];
    let (_, (mut i, mut j)) = state[s.len()][t.len()];
    while i < s.len() && j < t.len() {
        chars.push(s[i]);
        (i, j) = state[i][j].1;
    }
    chars.into_iter().rev().collect()
}

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let result = solve_dp(&s, &t);
    println!("{}", result.iter().join(""));
}
