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
    eprintln!("{}", Mint::new(9) / 17);
    for i in 0..n {
        v[i] = sum * p + p * (i + 1) * a[i];
        sum += v[i];
        result += v[i] * p * (i + 1);
        eprintln!("{}, {}, {}, {}, {}", p, a[i], p * a[i], sum, result);
    }
    println!("{result}");
}
