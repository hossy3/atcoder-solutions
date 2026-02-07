use proconio::input;

type Mint = ac_library::ModInt1000000007;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    // i 個の部屋を選ぶ: n C i
    let mut v0 = vec![Mint::new(1)]; // 組み合わせの個数
    for i in 0..n {
        let x = v0[i] * Mint::new(n - i) / Mint::new(i + 1);
        v0.push(x);
    }

    // (n-i) 個の部屋に計 i 個置く:
    // しきり (n-i-1) と i 個の合計から i を選ぶ: (n-1) C i
    let mut v1 = vec![Mint::new(1)]; // 組み合わせの個数
    for i in 0..n {
        let x = v1[i] * Mint::new(n - i - 1) / Mint::new(i + 1);
        v1.push(x);
    }

    // 掛け算すると答えになる
    let mut result = Mint::new(0);
    for i in 0..(n.min(k + 1)) {
        result += v0[i] * v1[i];
    }

    println!("{result}");
}
