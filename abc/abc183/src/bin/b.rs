use proconio::input;

fn main() {
    input! {
        sx: f64,
        sy: f64,
        gx: f64,
        gy: f64,
    }
    let x = sx + (gx - sx) * sy / (sy + gy);
    println!("{x}");
}
