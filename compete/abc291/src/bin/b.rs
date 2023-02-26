use proconio::input;

fn main() {
    input! {
        n: usize,
        mut x: [usize; n * 5],
    }
    x.sort();
    let mut sum = 0;
    for &y in &x[n..(n * 4)] {
        sum += y;
    }
    let result = (sum as f64) / (n as f64 * 3.0);
    println!("{}", result);
}
