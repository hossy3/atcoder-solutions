use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        h: f64,
        m: f64,
    }
    let rad0 = ((h * 60.0 + m) / (12.0 * 60.0) * 360.0).to_radians();
    let rad1 = (m / 60.0 * 360.0).to_radians();
    let x0 = a * rad0.cos();
    let y0 = a * rad0.sin();
    let x1 = b * rad1.cos();
    let y1 = b * rad1.sin();
    let result = ((x0 - x1).powi(2) + (y0 - y1).powi(2)).sqrt();
    println!("{result}");
}
