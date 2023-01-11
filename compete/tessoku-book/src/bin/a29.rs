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
        a: usize,
        b: usize,
    }
    const MOD: usize = 1000000007;

    let result = modpow(a, b, MOD);
    println!("{}", result);
}
