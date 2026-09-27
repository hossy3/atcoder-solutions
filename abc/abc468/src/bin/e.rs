use proconio::input;

type Mint = ac_library::ModInt998244353;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut sum = vec![Mint::new(0); n + 1]; // 1/i の累積和
    for i in 1..=n {
        sum[i] = sum[i - 1] + Mint::new(1) / Mint::new(i);
    }

    let mut result = Mint::new(0);
    let mut k = Mint::new(0);
    for i in 0..(n / 2) {
        k += sum[n - i] - sum[i];
        result += k * a[i];
        result += k * a[n - i - 1];
    }
    if n % 2 == 1 {
        k += sum[n / 2 + 1] - sum[n / 2];
        result += k * a[n / 2];
    }
    println!("{result}");
}
