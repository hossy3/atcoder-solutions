use proconio::input;

fn knapsack(n: usize, w_max: usize, wv: &[(usize, usize)]) -> usize {
    let mut dp = Vec::with_capacity(n + 1);
    dp.push(vec![None; w_max + 1]);
    dp[0][0] = Some(0);
    for (i, &(w, v)) in wv.iter().enumerate() {
        dp.push(dp.last().unwrap().clone());

        if w_max < w {
            continue;
        }
        for j in 0..=(w_max - w) {
            if let Some(v0) = dp[i][j] {
                let v1 = (v + v0).max(dp[i + 1][j + w].unwrap_or(0));
                dp[i + 1][j + w] = Some(v1);
            }
        }
    }

    let result = dp[n].iter().filter(|&&x| x != None).max().unwrap().unwrap();
    result
}

fn main() {
    input! {
        n: usize,
        w_max: usize,
        wv: [(usize, usize); n],
    }
    let result = knapsack(n, w_max, &wv);
    println!("{}", result);
}
