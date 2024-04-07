use itertools::Itertools;
use proconio::{input, marker::Usize1};

const N: usize = 8889;

fn dfs(
    i: usize,
    a: &[u16],
    deny: &[Vec<usize>],
    deny_counts: &mut Vec<u16>,
    sum: u16,
    cur: &mut Vec<usize>,
    results: &mut Vec<Vec<Vec<usize>>>,
) -> Option<Vec<Vec<usize>>> {
    if i == a.len() {
        let sum = sum as usize;
        results[sum].push(cur.clone());
        if results[sum].len() == 2 {
            return Some(results[sum].clone());
        } else {
            return None;
        }
    }

    // 選ばない
    let ret = dfs(i + 1, a, deny, deny_counts, sum, cur, results);
    if ret.is_some() {
        return ret;
    }

    // 選ぶ
    if deny_counts[i] > 0 {
        return None;
    }

    cur.push(i);
    for &j in &deny[i] {
        deny_counts[j] += 1;
    }
    let ret = dfs(i + 1, a, deny, deny_counts, sum + a[i], cur, results);
    for &j in &deny[i] {
        deny_counts[j] -= 1;
    }
    cur.pop();
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

    let mut deny = vec![vec![]; N];
    for &(x, y) in &xy {
        deny[x].push(y);
    }
    let mut deny_counts = vec![0; N];
    let mut cur = vec![];
    let mut results = vec![vec![]; N];
    let results = dfs(0, &a, &deny, &mut deny_counts, 0, &mut cur, &mut results).unwrap();

    for results in results {
        println!("{}", results.len());
        println!("{}", results.iter().map(|&x| x + 1).join(" "));
    }
}
