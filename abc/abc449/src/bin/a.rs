use std::f64;

use proconio::input;

fn main() {
    input! {
        d: f64,
    }
    let pi = f64::consts::PI;
    println!("{}", (d / 2.0) * (d / 2.0) * pi);
}
