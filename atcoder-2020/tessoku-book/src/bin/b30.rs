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

fn modinv(a: usize, m: usize) -> usize {
    modpow(a, m - 2, m)
}

fn modfact(n: usize, m: usize) -> usize {
    let mut x = 1;
    for i in 1..=n {
        x = (x * i) % m;
    }
    x
}

fn main() {
    input! {
        h: usize,
        w: usize,
    }
    const MOD: usize = 1000000007;
    let result = (((modfact(h + w - 2, MOD) * modinv(modfact(h - 1, MOD), MOD)) % MOD)
        * modinv(modfact(w - 1, MOD), MOD))
        % MOD;
    println!("{}", result);
}
