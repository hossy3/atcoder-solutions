use proconio::input;

// a + b * x <= c * x * d
// x >= a / (c * d - b)

fn main() {
    input! {
        a: f64,
        b: f64,
        c: f64,
        d: f64,
    }
    if b >= c * d {
        println!("{}", -1);
        return;
    }
    let x = (a / (c * d - b)).ceil();
    println!("{}", x);
}
