use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
    }
    let mut cum = vec![0i64; n + 1];
    for i in 0..n {
        cum[i + 1] = cum[i] + a[i];
    }
    let mut x = 0;
    for i in 0..m {
        x += ((i + 1) as i64) * a[i];
    }
    let mut score = x;
    for i in 0..(n - m) {
        x -= cum[i + m];
        x += cum[i];
        x += a[i + m] * (m as i64);
        score = score.max(x);
    }
    println!("{}", score);
}
