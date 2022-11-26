use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    }
    let mut k0 = 0;
    let mut k1 = a / b;
    let mut a0 = a as f64;
    while (k0 - k1).abs() > 2 {
        let k2 = (k0 * 2 + k1) / 3;
        let k3 = (k0 + k1 * 2) / 3;
        let a2 = (a as f64) / ((k2 + 1) as f64).sqrt() + (b * k2) as f64;
        let a3 = (a as f64) / ((k3 + 1) as f64).sqrt() + (b * k3) as f64;
        if a2 < a3 {
            k1 = k3;
        } else {
            a0 = a2;
            k0 = k2;
        }
    }

    let score = a0
        .min((a as f64) / ((k0 + 2) as f64).sqrt() + (b * (k0 + 1)) as f64)
        .min((a as f64) / ((k0 + 3) as f64).sqrt() + (b * (k0 + 2)) as f64);
    println!("{}", score);
}
