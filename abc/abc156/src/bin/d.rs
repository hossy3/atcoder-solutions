use proconio::input;

type Mint = ac_library::ModInt1000000007;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }

    let mut v = vec![Mint::new(1)]; // 組み合わせの個数
    for i in 0..(n.min(200_000)) {
        let x = v[i] * Mint::new(n - i) / Mint::new(i + 1);
        v.push(x);
    }

    let result = Mint::new(2).pow(n as u64) - v[a] - v[b] - 1;
    println!("{result}");
}
