use proconio::input;

fn main() {
    input! {
        n: usize,
        mut c: [i64; n],
    }
    const MOD: i64 = 1_000_000_007;
    c.sort();
    let result = (0..n).fold(1, |acc, x| (acc * (c[x] - x as i64).max(0)) % MOD);
    println!("{}", result);
}
