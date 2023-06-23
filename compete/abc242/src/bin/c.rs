use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    const MOD: usize = 998244353;
    let mut v = [1usize; 9];
    for _ in 1..n {
        let mut v0 = v.clone();
        for i in 0..8 {
            v0[i] += v[i + 1];
            v0[i + 1] += v[i];
        }
        for i in 0..9 {
            v0[i] %= MOD;
        }
        v = v0;
    }
    let result = v.iter().sum::<usize>() % MOD;
    println!("{}", result);
}
