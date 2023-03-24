use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut expected = 0.0;
    for i in 0..n {
        expected += (n as f64) / ((n - i) as f64);
    }
    println!("{}", expected);
}
