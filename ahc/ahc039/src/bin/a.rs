use std::time::Instant;

use itertools::Itertools;
use proconio::input;

const M: usize = 200;
const K: usize = 100_000 / M;
const LIMIT_TIME: f64 = 1.9;

fn find_bounding_box(m: &[Vec<i32>], mut x0: usize, mut y0: usize) -> (usize, usize, usize, usize) {
    let (mut x1, mut y1) = (x0 + 1, y0 + 1);
    loop {
        // 1つだけ伸ばすところを調べる
        loop {
            let mut cand = vec![];
            if x0 > 0 {
                let diff: i32 = (y0..y1).map(|y| m[x0 - 1][y]).sum();
                if diff >= 0 {
                    cand.push((diff, x0 - 1, y0, x1, y1));
                }
            }
            if y0 > 0 {
                let diff: i32 = (x0..x1).map(|x| m[x][y0 - 1]).sum();
                if diff >= 0 {
                    cand.push((diff, x0, y0 - 1, x1, y1));
                }
            }
            if x1 < M {
                let diff: i32 = (y0..y1).map(|y| m[x1][y]).sum();
                if diff >= 0 {
                    cand.push((diff, x0, y0, x1 + 1, y1));
                }
            }
            if y1 < M {
                let diff: i32 = (x0..x1).map(|x| m[x][y1]).sum();
                if diff >= 0 {
                    cand.push((diff, x0, y0, x1, y1 + 1));
                }
            }

            cand.sort();
            if let Some((_, x2, y2, x3, y3)) = cand.pop() {
                (x0, y0, x1, y1) = (x2, y2, x3, y3);
            } else {
                break;
            }
        }

        // 1つだけ伸ばしてもダメな場合は、複数伸ばして条件を満たすか調べる
        let mut cand = vec![];

        let mut diff = 0i32;
        for x2 in (0..x0).rev() {
            diff += (y0..y1).map(|y| m[x2][y]).sum::<i32>();
            if diff > 0 {
                cand.push((x0 - x2, x2, y0, x1, y1));
                break;
            }
        }
        diff = 0i32;
        for y2 in (0..y0).rev() {
            diff += (x0..x1).map(|x| m[x][y2]).sum::<i32>();
            if diff > 0 {
                cand.push((y0 - y2, x0, y2, x1, y1));
                break;
            }
        }
        diff = 0i32;
        for x2 in x1..M {
            diff += (y0..y1).map(|y| m[x2][y]).sum::<i32>();
            if diff > 0 {
                cand.push((x2 - x1 + 1, x0, y0, x2 + 1, y1));
                break;
            }
        }
        diff = 0i32;
        for y2 in y1..M {
            diff += (x0..x1).map(|x| m[x][y2]).sum::<i32>();
            if diff > 0 {
                cand.push((y2 - y0 + 1, x0, y0, x1, y2 + 1));
                break;
            }
        }

        cand.sort();
        if let Some(&(_, x2, y2, x3, y3)) = cand.first() {
            (x0, y0, x1, y1) = (x2, y2, x3, y3);
        } else {
            break;
        }
    }

    (x0, y0, x1, y1)
}

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); 2 * n],
    }

    // 縦横分割した区画に対して、その中にサバ・イワシが何匹いるか調べる
    let mut m = vec![vec![0i32; M]; M];
    for (x, y) in &xy[..n] {
        let (x0, y0) = (x / K, y / K); // 0..M の範囲にする
        m[x0][y0] += 1;
    }
    for (x, y) in &xy[n..] {
        let (x0, y0) = (x / K, y / K);
        m[x0][y0] -= 1;
    }

    // もっとも大きな区画
    let mut v = vec![];
    for x in 0..M {
        for y in 0..M {
            if m[x][y] > 0 {
                v.push((m[x][y], x, y));
            }
        }
    }

    let (mut x0, mut y0, mut x1, mut y1) = (0usize, 0usize, 0usize, 0usize);
    let mut score = i32::MIN;
    let timer = Instant::now();

    for &(_, x, y) in v.iter().sorted().rev() {
        if timer.elapsed().as_secs_f64() > LIMIT_TIME {
            break;
        }
        let (x2, y2, x3, y3) = find_bounding_box(&m, x, y);
        let score23 = (x2..x3)
            .map(|x| (y2..y3).map(|y| m[x][y]).sum::<i32>())
            .sum::<i32>();
        if score < score23 {
            (x0, y0, x1, y1, score) = (x2, y2, x3, y3, score23);
        }
    }

    println!("4");
    println!("{} {}", x0 * K, y0 * K);
    println!("{} {}", x1 * K, y0 * K);
    println!("{} {}", x1 * K, y1 * K);
    println!("{} {}", x0 * K, y1 * K);
}
