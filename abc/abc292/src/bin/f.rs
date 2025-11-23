use num::Float;
use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
    }

    let k = 2.0 / 3.0.sqrt();
    if a > b * k {
        println!("{}", b * k);
        return;
    } else if b > a * k {
        println!("{}", a * k);
        return;
    }

    let mut l = 0.0;
    let mut r = b * k * 0.5;
    while r - l > 1e-10 {
        let m = (l + r) / 2.0;
        let len = (m * m + b * b).sqrt();
        let b0 = (len * len - a * a).sqrt();
        let b1 = b - b0;
        let a1 = a - m;
        let len1 = (a1 * a1 + b1 * b1).sqrt();
        if len1 < len {
            r = m;
        } else {
            l = m;
        }
    }
    let m = (l + r) / 2.0;
    let len = (m * m + b * b).sqrt();
    println!("{}", len);
}
