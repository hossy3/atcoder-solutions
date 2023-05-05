use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }
    let i = (0..(n - 1)).position(|i| h[i] >= h[i + 1]).unwrap_or(n - 1);
    let result = h[i];
    println!("{}", result);
}
