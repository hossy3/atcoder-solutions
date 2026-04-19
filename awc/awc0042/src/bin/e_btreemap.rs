use std::collections::BTreeSet;

use proconio::input;

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
    let mut set0 = BTreeSet::new(); // 連休中の最大スコア - cum[i] (連続出勤に切り替えるときの計算用)
    let mut set1 = BTreeSet::new(); // 連勤中の最大スコア
    for i in 0..=n {
        if i >= k {
            let i = i - k; // k - 1 日までは連続で働ける
            set0.remove(&(v0[i] - cum[i], i));
        }
        if i >= m {
            let i = i - m; // m - 1 日までは連続で休める
            set1.remove(&(v1[i], i));
        }
        v0[i] = set1.last().unwrap_or(&(0, 0)).0;
        v1[i] = set0.last().unwrap_or(&(0, 0)).0 + cum[i];
        set0.insert((v0[i] - cum[i], i));
        set1.insert((v1[i], i));
    }

    let result = v0[n].max(v1[n]);
    println!("{result}");
}
