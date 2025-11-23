use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        mut a: [usize; n],
    }

    a.sort();

    // a の連続する数を組み立てる
    let mut v = vec![];
    for i in 0..n {
        if i == 0 || a[i] != a[i - 1] {
            v.push((i, a[i]));
        }
    }

    let mut dp = vec![1usize; n + 1];

    for i in 0..n {
        // dp の中で一番大きな値になれる
        let x = a[i];
        let mut j = v
            .binary_search_by(|x| x.0.cmp(&(i + 1)))
            .unwrap_or_else(|x| x);
        let mut k = 0; // 使用数
        while j < v.len() && k < 2 {
            let (i0, x0) = v[j]; // 番号, 値
            if x.abs_diff(x0) != d {
                dp[i0] = dp[i0].max(dp[i] + 1);
                k += 1;
            }
            j += 1;
        }
    }

    // 最大値が答え
    let result = n - dp.iter().max().unwrap();
    println!("{result}");
}
