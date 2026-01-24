use proconio::input;

type Mint = ac_library::ModInt1000000007;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut v = vec![Mint::new(0); k + 1]; // gcd が k になる個数
    for i in (1..=k).rev() {
        let mut x = Mint::new(k / i).pow(n as u64);
        for j in ((i * 2)..=k).step_by(i) {
            x -= v[j]; // gcd は i ではなく j なので除く
        }
        v[i] = x;
    }

    let result: Mint = (1..=k).map(|i| v[i] * i).sum();
    println!("{result}");
}
