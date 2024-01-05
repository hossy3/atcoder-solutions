use proconio::input;

fn main() {
    input! {
        a: usize,
        b: f64,
    }
    let b0 = (b * 100.0).round() as usize;
    let result = (a * b0) / 100;
    println!("{result}");
}
