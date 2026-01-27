use proconio::input;

fn main() {
    input! {
        k: usize,
        n: usize,
        a: [usize; n],
    }
    let result = k - a
        .windows(2)
        .map(|v| v[1] - v[0])
        .max()
        .unwrap_or(0)
        .max(a[0] + k - a[n - 1]);
    println!("{result}");
}
