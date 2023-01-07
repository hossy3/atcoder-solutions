use proconio::input;

fn modpow(n: usize, k: usize, m: usize) -> usize {
    match k {
        0 => 1,
        1 => n,
        _ => {
            let x = modpow(n, k / 2, m);
            if k % 2 == 0 {
                (x * x) % m
            } else {
                (((x * x) % m) * n) % m
            }
        }
    }
}

fn main() {
    input! {
        w: usize,
    }
    const MOD: usize = 1000000007;

    let result = (4 * 3 * modpow(2 + 2 + 3, w - 1, MOD)) % MOD;
    println!("{}", result);
}
