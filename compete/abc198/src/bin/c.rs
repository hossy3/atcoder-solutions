use proconio::input;

fn main() {
    input! {
        r: usize,
        x: usize,
        y: usize,
    }
    if (x * x + y * y < r * r) && x + y > 0 {
        println!("{}", 2);
    } else {
        let len = ((x * x + y * y) as f64).sqrt();
        let result = (len / (r as f64)).ceil();
        println!("{}", result);
    }
}
