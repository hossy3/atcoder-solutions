use std::collections::VecDeque;

use proconio::input;

// スライド最小値・スライド最大値
fn push_max(deque: &mut VecDeque<(usize, isize)>, i: usize, value: isize) {
    while deque.len() > 0 && deque[deque.len() - 1].1 <= value {
        deque.pop_back();
    }
    deque.push_back((i, value));
}

fn main() {
    input! {
        n: usize,
        k: usize,
        m: usize,
        a: [isize; n],
    }

    let mut cum = vec![0isize; n + 1];
    for (i, &a) in a.iter().enumerate() {
        cum[i + 1] = cum[i] + a;
    }

    let mut v0 = vec![0isize; n + 1]; // i 日目が連休最終日のときの最大スコア
    let mut v1 = vec![0isize; n + 1]; // i 日目が連勤最終日のときの最大スコア
    let mut d0 = VecDeque::new(); // (i, 連休中の最大スコア - cum[i]) (連続出勤に切り替えるときの計算用)
    let mut d1 = VecDeque::new(); // (i, 連勤中の最大スコア)
    d0.push_back((0, 0));
    d1.push_back((0, 0));

    for i in 1..=n {
        while i >= k && d0[0].0 <= i - k {
            d0.pop_front();
        }
        while i >= m && d1[0].0 <= i - m {
            d1.pop_front();
        }
        v0[i] = d1[0].1;
        v1[i] = d0[0].1 + cum[i];
        push_max(&mut d0, i, v0[i] - cum[i]);
        push_max(&mut d1, i, v1[i]);
    }

    let result = v0[n].max(v1[n]);
    println!("{result}");
}
