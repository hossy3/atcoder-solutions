use proconio::input;

type Mint = ac_library::ModInt1000000007;

fn main() {
    input! {
        n: usize,
    }
    let mut v = vec![Mint::new(0); n + 1];
    let mut v0 = vec![Mint::new(0); n + 1];
    let mut v9 = vec![Mint::new(0); n + 1];
    let mut v09 = vec![Mint::new(0); n + 1];
    v[0] = Mint::new(1);
    for i in 0..n {
        v[i + 1] = v[i] * 8;
        v0[i + 1] = v[i] + v0[i] * 9;
        v9[i + 1] = v[i] + v9[i] * 9;
        v09[i + 1] = v0[i] + v9[i] + v09[i] * 10;
    }
    let result = v09[n];
    println!("{result}");
}
