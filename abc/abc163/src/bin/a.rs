use std::f64::consts::PI;

use proconio::input;

fn main() {
    input! {
        r: f64,
    }
    let result = 2.0 * PI * r;
    println!("{result}");
}
