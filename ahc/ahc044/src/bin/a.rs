use std::cmp::Reverse;
use std::iter::zip;
use std::time::Instant;

use itertools::Itertools;
use proconio::input;
use rand::Rng;

const LIMIT_TIME: f64 = 10.9;

fn simulate(l: usize, start: usize, followings: &[(usize, usize)]) -> Vec<usize> {
    let mut counts = vec![0usize; followings.len()];
    let mut cur = start;
    for _ in 0..l {
        counts[cur] += 1;
        if counts[cur] % 2 == 1 {
            cur = followings[cur].0;
        } else {
            cur = followings[cur].1;
        }
    }
    counts
}

fn calc_score(t0: &[(usize, usize)], simulated: &[usize]) -> usize {
    zip(t0.iter(), simulated.iter())
        .map(|((_, t), &s)| t.abs_diff(s))
        .sum()
}

fn main() {
    input! {
        n: usize,
        l: usize,
        t: [usize; n],
    }

    // 初期配置はまっすぐ延ばす
    let t0: Vec<_> = t
        .iter()
        .enumerate()
        .map(|(i, &x)| (i, x))
        .sorted_by_key(|&(_, x)| Reverse(x))
        .collect();
    let Some((start, _)) = t0.iter().find_position(|(i, _)| *i == 0) else {
        unreachable!()
    };

    let mut followings = vec![(0usize, 0usize); n];
    for i in 0..n {
        followings[i].0 = (i + 1) % n;
        followings[i].1 = (i + 1) % n;
    }

    let mut rng = rand::thread_rng();

    let timer = Instant::now();

    let simulated = simulate(l, start, &followings);
    let mut score = calc_score(&t0, &simulated);
    while timer.elapsed().as_secs_f64() < LIMIT_TIME * 0.5 {
        let i = rng.gen_range(0..n);
        let j = rng.gen_range(0..=(i + 1));
        if followings[i].1 == j {
            continue;
        }
        let x = followings[i].1; 
        followings[i].1 = j;

        let simulated = simulate(l, start, &followings);
        let score0 = calc_score(&t0, &simulated);
        if score0 > score {
            followings[i].1 = x;
            continue;
        }
        score = score0;
    }

    // 結果を組み立て、出力
    let mut results = vec![(0usize, 0usize); n];
    for (i, &(a, b)) in followings.iter().enumerate() {
        results[t0[i].0] = (t0[a].0, t0[b].0);
    }
    for (a, b) in results {
        println!("{a} {b}");
    }
}
