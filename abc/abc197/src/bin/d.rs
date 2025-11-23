use std::f64::consts::PI;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x0: f64,
        y0: f64,
        xm: f64,
        ym: f64,
    }
    let xc = (xm + x0) / 2.0;
    let yc = (ym + y0) / 2.0;
    let r = ((x0 - xc).powf(2.0) + (y0 - yc).powf(2.0)).sqrt();
    let t0 = (y0 - yc).atan2(x0 - xc);
    let t1 = t0 + PI * 2.0 / n as f64;
    let x1 = xc + r * t1.cos();
    let y1 = yc + r * t1.sin();
    println!("{x1} {y1}");
}
