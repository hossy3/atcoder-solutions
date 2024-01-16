use proconio::input;

type Mint = ac_library::ModInt1000000007;

trait BitTest {
    fn bit_test(&self, i: usize) -> bool;
}

impl BitTest for usize {
    fn bit_test(&self, i: usize) -> bool {
        self & (1 << i) != 0
    }
}

fn mul(k0: usize, v0: &[Mint], v1: &[Mint]) -> Vec<Mint> {
    let b = v0.len();
    let mut v = vec![Mint::new(0); b];

    for (i0, &x0) in v0.iter().enumerate() {
        for (i1, &b1) in v1.iter().enumerate() {
            let i = (i0 * k0 + i1) % b;
            v[i] += x0 * b1;
        }
    }

    v
}

fn main() {
    input! {
        n: usize,
        b: usize,
        k: usize,
        c: [usize; k],
    }

    let mut dp = vec![vec![Mint::new(0); b]];
    for &x in &c {
        let i = x % b;
        dp[0][i] += Mint::new(1);
    }

    let log2 = (usize::BITS - n.leading_zeros()) as usize;
    assert!(log2 > 0);
    let mut pow10s = vec![0; log2]; // [(10^(2^0)) % b, (10^(2^1)) % b, (10^(2^2)) % b, ...]
    pow10s[0] = 10 % b;
    for i in 0..(log2 - 1) {
        pow10s[i + 1] = (pow10s[i] * pow10s[i]) % b;
    }

    for i in 0..(log2 - 1) {
        dp.push(mul(pow10s[i], &dp[i], &dp[i]));
    }

    let mut results = vec![Mint::new(0); b];
    results[0] = Mint::new(1);
    for i in 0..log2 {
        if n.bit_test(i) {
            results = mul(pow10s[i], &results, &dp[i]);
        }
    }

    let result = results[0];
    println!("{result}");
}
