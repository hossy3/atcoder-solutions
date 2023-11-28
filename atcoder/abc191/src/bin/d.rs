use proconio::input;

fn main() {
    input! {
        x: f64,
        y: f64,
        r: f64,
    }
    let x0 = (x * 10000.0).round() as i64;
    let y0 = (y * 10000.0).round() as i64;
    let r0 = (r * 10000.0).round() as i64;

    let mut result = 0;
    let y1 = (y0 - r0) / 10000;
    let y2 = (y0 + r0) / 10000;
    for y in y1..=y2 {
        let h = (y0 - y * 10000).abs();
        if r0 < h {
            continue;
        }
        let mut w = ((r0 * r0 - h * h) as f64).sqrt() as i64;
        if w * w > r0 * r0 - h * h {
            w -= 1; // 計算誤差対策
        }
        let x1 = ((x0 - w) as f64 / 10000.0).ceil() as i64;
        let x2 = ((x0 + w) as f64 / 10000.0).floor() as i64;
        result += x2 - x1 + 1;
    }
    println!("{result}");
}
