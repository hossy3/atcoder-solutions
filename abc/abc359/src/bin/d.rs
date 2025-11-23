use proconio::{input, marker::Chars};

const MOD: usize = 998244353;

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
        s: Chars,
    }

    let k0 = 1 << k;
    let b: Vec<_> = (0..k0)
        .map(|i| (0..k).any(|j| i.bit_test(j) != i.bit_test(k - j - 1)))
        .collect();

    let mut dp = vec![0usize; k0];
    for i in 0..k0 {
        if !b[i] {
            continue;
        }
        if s[0..k]
            .iter()
            .enumerate()
            .all(|(j, &c)| (c == 'A' && i.bit_test(j)) || (c == 'B' && !i.bit_test(j) || c == '?'))
        {
            dp[i] = 1;
        }
    }

    for j in k..n {
        let prev = dp.clone();
        dp.fill(0);

        let c = s[j];

        for i in 0..k0 {
            if !b[i] {
                continue;
            }
            if (c == 'A' && i.bit_test(k - 1)) || (c == 'B' && !i.bit_test(k - 1) || c == '?') {
                let i0 = (i * 2) % k0;
                dp[i] = (prev[i0] + prev[i0 + 1]) % MOD;
            }
        }
    }

    let result = dp.iter().sum::<usize>() % MOD;
    println!("{result}");
}
