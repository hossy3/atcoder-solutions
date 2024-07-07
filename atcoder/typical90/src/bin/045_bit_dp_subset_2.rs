use proconio::input;

trait BitTest {
    fn bit_test(&self, i: usize) -> bool;
}

impl BitTest for usize {
    fn bit_test(&self, i: usize) -> bool {
        self & (1 << i) != 0
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
        xy: [(usize, usize); n],
    }

    let mut dist = vec![vec![0; n]; n];
    for (i0, &(x0, y0)) in xy.iter().enumerate() {
        for (i1, &(x1, y1)) in xy.iter().enumerate() {
            dist[i0][i1] = x1.abs_diff(x0).pow(2) + y1.abs_diff(y0).pow(2);
        }
    }

    let mut cost = vec![0usize; 1 << n];
    for i in 0..(1 << n) {
        for j0 in 0..n {
            for j1 in 0..j0 {
                if i.bit_test(j0) && i.bit_test(j1) {
                    cost[i] = cost[i].max(dist[j0][j1]);
                }
            }
        }
    }

    let mut dp = vec![vec![usize::MAX; 1 << n]; k + 1];
    dp[0][0] = 0;
    for i in 0..k {
        for j0 in 0..(1 << n) {
            // j0 を j1 と j2 に分割
            let mut j1 = j0;
            loop {
                let j2 = j0 ^ j1; // j0 - j1 でも同じ
                let x = dp[i][j2].max(cost[j1]);
                dp[i + 1][j0] = dp[i + 1][j0].min(x);
                if j1 == 0 {
                    break;
                }
                j1 = (j1 - 1) & j0;
            }
        }
    }

    let result = dp[k][(1 << n) - 1];
    println!("{result}");
}
