use proconio::input;

type Mint = ac_library::ModInt1000000007;

fn main() {
    input! {
        n: usize,
        mut c: [usize; n],
    }
    c.sort();

    let mut result = Mint::new(0);
    let base = Mint::new(2).pow(n as u64);
    for (i, &x) in c.iter().enumerate() {
        // 並び替えて小さな方から消していく
        // 残りの異なる数は、自分自身と、右側の個数の半分
        result += base * x * (Mint::new(2).pow((n - 1) as u64) * (2 + n - i - 1) / 2);
    }
    println!("{result}");
}
