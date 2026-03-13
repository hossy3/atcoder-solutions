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
        c: usize,
        w: [usize; n],
    }

    let mut costs = vec![0usize; 1 << n];
    for i in 0..(1 << n) {
        for j in 0..n {
            if i.bit_test(j) {
                costs[i] += w[j];
            }
        }
    }

    let mut dp = vec![usize::MAX; 1 << n];
    dp[0] = 0;
    for i in 0..(1 << n) {
        if dp[i] == usize::MAX {
            continue;
        }
        for j in bit_subset(((1 << n) - 1) ^ i) {
            if costs[j] > c {
                continue;
            }
            dp[i | j] = dp[i | j].min(dp[i] + 1);
        }
    }

    let result = dp[(1 << n) - 1];
    println!("{result}");
}
