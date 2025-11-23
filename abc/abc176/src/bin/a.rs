use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        t: usize,
    }
    let result = (n as f64 / x as f64).ceil() as usize * t;
    println!("{result}");
}
