use proconio::input;

fn main() {
    input! {
        h: f64,
    }
    let result = (h * (12800000.0 + h)).sqrt();
    println!("{}", result);
}
