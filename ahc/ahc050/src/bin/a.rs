use std::time::Instant;

use proconio::{input, marker::Chars};
use rand::{rngs::ThreadRng, Rng};

// 次のロボットの存在確率を返す
fn f(s: &Vec<Vec<char>>, p: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let n = s.len();
    let mut p0 = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            for dir in 0..4 {
                let (x, y) = pos(s, i, j, dir);
                p0[x][y] += p[i][j];
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            p0[i][j] = (p0[i][j] + 2) / 4;
        }
    }

    p0
}

// 一番ロボットが存在しないところを返す
fn g(s: &Vec<Vec<char>>, p: &Vec<Vec<usize>>, rng: &mut ThreadRng) -> (usize, usize) {
    let n = s.len();

    let mut v = vec![];

    for i in 0..n {
        for j in 0..n {
            if s[i][j] == '#' {
                continue;
            }
            if v.len() == 0 {
                v.push((i, j));
                continue;
            }
            let (i0, j0) = v[0];
            if p[i][j] < p[i0][j0] {
                v.clear();
                v.push((i, j));
            } else if p[i][j] == p[i0][j0] {
                v.push((i, j));
            }
        }
    }

    let i = rng.gen_range(0..(v.len()));
    v[i]
}

fn pos(s: &Vec<Vec<char>>, mut i: usize, mut j: usize, dir: usize) -> (usize, usize) {
    let n = s.len();
    match dir {
        0 => {
            while i > 0 && s[i - 1][j] != '#' {
                i -= 1;
            }
        }
        1 => {
            while i < n - 1 && s[i + 1][j] != '#' {
                i += 1;
            }
        }
        2 => {
            while j > 0 && s[i][j - 1] != '#' {
                j -= 1;
            }
        }
        3 => {
            while j < n - 1 && s[i][j + 1] != '#' {
                j += 1;
            }
        }
        _ => unreachable!(),
    }
    (i, j)
}

const LIMIT_TIME: f64 = 2.0;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }

    let mut rng = rand::thread_rng();
    let timer = Instant::now();

    let mut best_score = 0u128;
    let mut best_results = vec![];
    let mut trace_count = 0;

    while timer.elapsed().as_secs_f64() < LIMIT_TIME * 0.9 {
        let mut s: Vec<_> = s.iter().map(|s| s.clone()).collect(); // 盤面
        let mut p = vec![vec![usize::MAX / (4 * n * n); n]; n]; // ロボットの存在確率 (整数)
        let mut zero_count = 0;
        let mut zero_only = true;

        for i in 0..n {
            for j in 0..n {
                if s[i][j] == '#' {
                    p[i][j] = 0;
                }
            }
        }

        let mut score = 0;
        let mut results = vec![];
        for i in 0..(n * n - m) {
            p = f(&s, &p);
            let (i, j) = if i < trace_count {
                best_results[i]
            } else {
                g(&s, &p, &mut rng)
            };
            results.push((i, j));
            if zero_only && p[i][j] == 0 {
                zero_count += 1;
            } else {
                zero_only = false;
            }
            s[i][j] = '#';
            p[i][j] = 0;
            score += p
                .iter()
                .map(|v| v.iter().map(|&x| x as u128).sum::<u128>())
                .sum::<u128>();
        }

        if score > best_score {
            best_score = score;
            best_results = results;
        }

        let elapsed = timer.elapsed().as_secs_f64();
        trace_count = if elapsed < LIMIT_TIME * 0.25 {
            0
        } else if elapsed < LIMIT_TIME * 0.5 {
            zero_count - (rng.gen_range(0..(zero_count * zero_count)) as f64).sqrt() as usize - 1
        } else if elapsed < LIMIT_TIME * 0.75 {
            rng.gen_range(0..zero_count)
        } else {
            (rng.gen_range(0..(zero_count * zero_count)) as f64).sqrt() as usize
        };
    }

    for (i, j) in best_results {
        println!("{i} {j}");
    }
}
