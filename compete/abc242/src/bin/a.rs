use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize,
    }
    let p = if x <= a {
        1.0
    } else if x <= b {
        (c as f64) / ((b - a) as f64)
    } else {
        0.0
    };
    println!("{}", p);
}
