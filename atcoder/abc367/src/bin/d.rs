use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    // 累積和を2周分求める
    let mut v = vec![0usize; n * 2 + 1];
    for i in 0..(2 * n) {
        v[i + 1] = (v[i] + a[i % n]) % m;
    }

    // mの倍数の個数を数える
    let mut counts = vec![0usize; m];
    for i in 0..n {
        counts[v[i]] += 1;
    }
    let mut result = 0usize;
    for i in 0..n {
        result += counts[v[i]] - 1; // 0歩は含めない
        counts[v[i + n]] += 1;
        counts[v[i]] -= 1;
    }
    println!("{result}");
}
