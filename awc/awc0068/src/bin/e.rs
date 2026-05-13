use std::collections::VecDeque;

use proconio::input;

/// スライド最小値・スライド最大値 (Sliding Window Aggregation, SWAG)
fn slide_step(width: usize, (i, value): (usize, usize), deque: &mut VecDeque<(usize, usize)>) {
    while deque.len() > 0 && i - deque[0].0 >= width {
        deque.pop_front();
    }
    while deque.len() > 0 && deque[deque.len() - 1].1 <= value {
        deque.pop_back();
    }
    deque.push_back((i, value));
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [[usize; n]; n],
    }

    // k*k の和
    let sum = {
        let mut sum = vec![vec![0usize; a[0].len() + 1]; a.len() + 1];
        for (i, a) in a.iter().enumerate() {
            for (j, &a) in a.iter().enumerate() {
                sum[i + 1][j + 1] = sum[i][j + 1] + sum[i + 1][j] - sum[i][j] + a;
            }
        }
        sum
    };

    // k*k の枠内の最大値
    let mut m0 = vec![vec![0usize; n - k + 1]; n];
    for i in 0..n {
        let mut deque = VecDeque::new();
        for j in 0..n {
            slide_step(k, (j, a[i][j]), &mut deque);
            if j + 1 >= k {
                let x = deque[0].1;
                m0[i][j + 1 - k] = x;
            }
        }
    }

    let mut m = vec![vec![0usize; n - k + 1]; n - k + 1];
    for i in 0..(n - k + 1) {
        let mut deque = VecDeque::new();
        for j in 0..n {
            slide_step(k, (j, m0[j][i]), &mut deque);
            if j + 1 >= k {
                let x = deque[0].1;
                m[j + 1 - k][i] = x;
            }
        }
    }

    let mut result = 0;
    for i in 0..(n - k + 1) {
        for j in 0..(n - k + 1) {
            let x = sum[i + k][j + k] + sum[i][j] - (sum[i][j + k] + sum[i + k][j]) - m[i][j];
            result = result.max(x);
        }
    }

    println!("{result}");
}
