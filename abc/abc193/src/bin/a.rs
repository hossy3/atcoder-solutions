use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
    }
    let result = (1.0 - b / a) * 100.0;
    println!("{}", result);
}
