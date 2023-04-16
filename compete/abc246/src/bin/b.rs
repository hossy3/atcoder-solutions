use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
    }
    let len = (a * a + b * b).sqrt();
    println!("{} {}", a / len, b / len);
}
