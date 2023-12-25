use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [f64; n],
    }
    let m: f64 = x.iter().map(|x| x.abs()).sum();
    let e = x.iter().map(|x| x * x).sum::<f64>().sqrt();
    let c = x.iter().fold(0f64, |acc, x| acc.max(x.abs()));
    println!("{m}");
    println!("{e}");
    println!("{c}");
}
