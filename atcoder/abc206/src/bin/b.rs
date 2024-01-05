use proconio::input;

// x * (x + 1) / 2 >= n
// x * x + x - 2n >= 0
// x >= (-1 + sqrt(1 + 8n)) / 2

fn main() {
    input! {
        n: f64,
    }
    let x = ((-1.0 + (-1.0 + 8.0 * n).sqrt()) / 2.0).ceil();
    println!("{}", x);
}
