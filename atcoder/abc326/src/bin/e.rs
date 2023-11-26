use proconio::input;

type Mint = ac_library::ModInt998244353;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut v = vec![Mint::new(0); n];
    let mut result = Mint::new(0);
    let mut sum = Mint::new(0);
    let p = Mint::new(1) / n;
    for i in 0..n {
        v[i] = sum * p + p;
        sum += v[i];
        result += v[i] * a[i];
    }
    println!("{result}");
}
