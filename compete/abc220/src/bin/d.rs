use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    const MOD: usize = 998244353;
    let mut v = vec![vec![0usize; 10]; n];
    v[0][a[0]] = 1;
    for i in 1..n {
        for j in 0..10 {
            let i0 = (j + a[i]) % 10;
            let i1 = (j * a[i]) % 10;
            v[i][i0] = (v[i][i0] + v[i - 1][j]) % MOD;
            v[i][i1] = (v[i][i1] + v[i - 1][j]) % MOD;
        }
    }
    for &x in &v[n - 1] {
        println!("{}", x);
    }
}
