use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
        e: i64,
        f: i64,
    }
    const MOD: i64 = 998244353;
    let a = a % MOD;
    let b = b % MOD;
    let c = c % MOD;
    let d = d % MOD;
    let e = e % MOD;
    let f = f % MOD;
    let x = (((a * b) % MOD) * c) % MOD;
    let y = (((d * e) % MOD) * f) % MOD;
    let result = (x - y + MOD) % MOD;
    println!("{}", result);
}
