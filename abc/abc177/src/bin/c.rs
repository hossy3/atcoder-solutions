use proconio::input;

type Mint = ac_library::ModInt1000000007;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut v = a.clone();
    for i in 1..n {
        v[i] += v[i - 1];
    }
    let result = (1..n).fold(Mint::new(0), |acc, x| {
        acc + Mint::new(v[x - 1]) * Mint::new(a[x])
    });
    println!("{result}");
}
