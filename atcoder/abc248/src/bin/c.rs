use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }
    const MOD: usize = 998244353;

    let mut v = vec![0usize; n * m + 1];
    v[0] = 1;
    for i in 0..n {
        let mut v0 = vec![0usize; n * m + 1];
        for j in 0..=(i * m) {
            for k in 1..=m {
                v0[j + k] += v[j];
            }
        }
        for j in (i + 1)..=((i + 1) * m) {
            v0[j] = v0[j] % MOD;
        }
        v = v0;
    }

    let result = v[0..=k].iter().sum::<usize>() % MOD;
    println!("{}", result);
}
