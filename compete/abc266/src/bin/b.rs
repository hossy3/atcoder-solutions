use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    const MOD: i64 = 998244353;
    let x = ((n % MOD) + MOD) % MOD;
    println!("{}", x);
}
