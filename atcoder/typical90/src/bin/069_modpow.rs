use proconio::input;

const MOD: usize = 1_000_000_007;

fn modpow(n: usize, k: usize) -> usize {
    if k == 1 {
        n
    } else {
        let k0 = k / 2;
        let x = modpow(n, k0);
        if k % 2 == 1 {
            (((x * x) % MOD) * n) % MOD
        } else {
            (x * x) % MOD
        }
    }
}

fn f(n: usize, k: usize) -> usize {
    if k == 1 {
        if n == 1 {
            1
        } else {
            0
        }
    } else {
        if n == 1 {
            k
        } else if n == 2 {
            (k * (k - 1)) % MOD
        } else {
            (((k * (k - 1)) % MOD) * (modpow(k - 2, n - 2) % MOD)) % MOD
        }
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let count = f(n, k);
    println!("{count}");
}
