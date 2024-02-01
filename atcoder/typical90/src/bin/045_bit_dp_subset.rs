use proconio::input;

trait BitTest {
    fn bit_test(&self, i: usize) -> bool;
}

impl BitTest for usize {
    fn bit_test(&self, i: usize) -> bool {
        self & (1 << i) != 0
    }
}

struct BitSubset {
    start: usize,
    next: Option<usize>,
}

impl Iterator for BitSubset {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.next;
        if let Some(x) = self.next {
            self.next = if x > 0 {
                Some((x - 1) & self.start)
            } else {
                None
            };
        }
        cur
    }
}

fn bit_subset(start: usize) -> BitSubset {
    BitSubset {
        start,
        next: Some(start),
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
            for j1 in bit_subset(j0) {
                let x = dp[i][j0 ^ j1].max(cost[j1]);
                dp[i + 1][j0] = dp[i + 1][j0].min(x);
            }
        }
    }

    let result = dp[k][(1 << n) - 1];
    println!("{result}");
}
