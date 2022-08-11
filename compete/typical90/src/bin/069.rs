use proconio::input;

const MOD: usize = 1_000_000_007;

fn f(n: usize, k: usize) -> usize {
    if n == 1 {
        k
    } else {
        let m = n / 2;
        let x = f(m, k);
        if n % 2 == 1 {
            (((x * x) % MOD) * k) % MOD
        } else {
            (x * x) % MOD
        }
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    // corner case
    if n == 1 {
        println!("{}", k);
        return;
    }
    if n == 2 {
        println!("{}", k * (k - 1));
        return;
    }
    if n > 2 && k < 3 {
        println!("{}", 0);
        return;
    }

    let count = (((k * (k - 1)) % MOD) * (f(n - 2, k - 2) % MOD)) % MOD;
    println!("{}", count);
}
