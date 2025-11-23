use std::{collections::VecDeque, time::Instant};

use itertools::Itertools;
use proconio::{input, marker::Chars};
use rand::{rngs::ThreadRng, Rng};

// !map[i][j] となる最短距離を見つける
fn dijkstra((i, j): (usize, usize), map: &[Vec<bool>], v: &[Vec<char>], h: &[Vec<char>]) -> usize {
    let n = map.len();
    let mut walked = vec![vec![false; n]; n];
    let mut queue = VecDeque::new();
    queue.push_back((0, i, j));
    while let Some((step, i, j)) = queue.pop_front() {
        if !map[i][j] {
            return step;
        }
        if walked[i][j] {
            continue;
        }
        walked[i][j] = true;
        let step = step + 1;
        if i > 0 && h[i - 1][j] == '0' {
            queue.push_back((step, i - 1, j));
        };
        if i < n - 1 && h[i][j] == '0' {
            queue.push_back((step, i + 1, j));
        };
        if j > 0 && v[i][j - 1] == '0' {
            queue.push_back((step, i, j - 1));
        };
        if j < n - 1 && v[i][j] == '0' {
            queue.push_back((step, i, j + 1));
        };
    }

    unreachable!()
}

const LIMIT_TIME: f64 = 19.9;

fn f(
    n: usize,
    m: usize,
    k: usize,
    mut ij: Vec<(usize, usize)>,
    v: &[Vec<char>],
    h: &[Vec<char>],
    timer: &Instant,
    rng: &mut ThreadRng,
) -> (bool, Vec<Vec<usize>>, Vec<usize>) {
    let mut ops = vec![vec![0; m]; k]; // m台のロボット, k種類の操作, UDLR
    for i in 0..=3 {
        ops[i] = vec![i; m];
    }
    for i in 4..k {
        for j in 0..m {
            ops[i][j] = rng.gen_range(0..4);
        }
    }

    let mut map = vec![vec![false; n]; n]; // きれいにしたマップ
    for &(i, j) in &ij {
        map[i][j] = true;
    }

    let mut results = vec![];
    while map.iter().any(|row| row.iter().any(|x| !x)) {
        let mut candidates = vec![];
        let mut candidates_ij = vec![vec![]; k];
        for k0 in 0..k {
            let mut cand = vec![];
            for m0 in 0..m {
                let (i, j) = ij[m0];
                let (i, j) = match ops[k0][m0] {
                    0 => {
                        if i > 0 && h[i - 1][j] == '0' {
                            (i - 1, j)
                        } else {
                            (i, j)
                        }
                    }
                    1 => {
                        if i < n - 1 && h[i][j] == '0' {
                            (i + 1, j)
                        } else {
                            (i, j)
                        }
                    }
                    2 => {
                        if j > 0 && v[i][j - 1] == '0' {
                            (i, j - 1)
                        } else {
                            (i, j)
                        }
                    }
                    3 => {
                        if j < n - 1 && v[i][j] == '0' {
                            (i, j + 1)
                        } else {
                            (i, j)
                        }
                    }
                    _ => unreachable!(),
                };
                let score = dijkstra((i, j), &map, &v, &h);
                cand.push(score);

                candidates_ij[k0].push((i, j));
            }

            cand.sort();
            candidates.push((cand, k0));
        }

        candidates.sort();
        let k = candidates[0].1;
        results.push(k);
        ij = candidates_ij[k].clone();

        for &(i, j) in &ij {
            map[i][j] = true;
        }

        if results.len() >= 2 * n * n || timer.elapsed().as_secs_f64() >= LIMIT_TIME {
            return (false, ops, results);
        }
    }

    (true, ops, results)
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        mut ij: [(usize, usize); m],
        v: [Chars; n],
        h: [Chars; n - 1],
    }

    let mut rng = rand::thread_rng();
    let timer = Instant::now();

    let mut ops = vec![]; // m台のロボット, k種類の操作, UDLR
    let mut results = vec![];
    loop {
        let (ok, ops0, results0) = f(n, m, k, ij.clone(), &v, &h, &timer, &mut rng);
        if !ok && !results.is_empty() {
            break;
        }
        if results.is_empty() || results.len() > results0.len() {
            ops = ops0;
            results = results0;
        }
    }

    // 出力
    for ops in &ops {
        let result = ops
            .iter()
            .map(|&x| match x {
                0 => 'U',
                1 => 'D',
                2 => 'L',
                3 => 'R',
                _ => unreachable!(),
            })
            .join(" ");
        println!("{result}");
    }

    for result in results {
        println!("{result}");
    }
}
