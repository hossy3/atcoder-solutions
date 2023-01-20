use proconio::input;

fn main() {
    input! {
        n: f64,
    }
    let mut l = 0.0;
    let mut r = (n as f64).powf(1.0 / 3.0);
    while r - l > 1e-6 {
        let x = (l + r) / 2.0;
        if x * x * x + x > n {
            r = x;
        } else {
            l = x;
        }
    }

    let result = (l + r) / 2.0;
    println!("{:.6}", result);
}
