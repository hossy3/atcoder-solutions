use itertools::Itertools;
use proconio::{input, marker::Usize1};

const N: usize = 8889;

struct Input {
    a: Vec<u16>,
    deny: Vec<Vec<usize>>,
}

struct State {
    cur: Vec<usize>,
    deny_counts: Vec<u16>,
    results: Vec<Vec<Vec<usize>>>,
}

fn dfs(i: usize, sum: u16, input: &Input, state: &mut State) -> Option<Vec<Vec<usize>>> {
    if i == input.a.len() {
        let sum = sum as usize;
        let cur = state.cur.clone();
        state.results[sum].push(cur);
        if state.results[sum].len() == 2 {
            return Some(state.results[sum].clone());
        } else {
            return None;
        }
    }

    // 選ばない
    let ret = dfs(i + 1, sum, input, state);
    if ret.is_some() {
        return ret;
    }

    // 選ぶ
    if state.deny_counts[i] > 0 {
        return None;
    }

    state.cur.push(i);
    for &j in &input.deny[i] {
        state.deny_counts[j] += 1;
    }
    let ret = dfs(i + 1, sum + input.a[i], input, state);
    for &j in &input.deny[i] {
        state.deny_counts[j] -= 1;
    }
    state.cur.pop();
    if ret.is_some() {
        return ret;
    }

    None
}

fn main() {
    input! {
        (n, q): (usize, usize),
        a: [u16; n],
        xy: [(Usize1, Usize1); q],
    }

    let mut input = Input {
        a,
        deny: vec![vec![]; N],
    };
    for &(x, y) in &xy {
        input.deny[x].push(y);
    }

    let mut state = State {
        cur: vec![],
        deny_counts: vec![0; N],
        results: vec![vec![]; N],
    };
    let results = dfs(0, 0, &input, &mut state).unwrap();

    for results in results {
        println!("{}", results.len());
        println!("{}", results.iter().map(|&x| x + 1).join(" "));
    }
}
