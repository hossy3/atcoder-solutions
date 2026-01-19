use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        n: usize,
    }
    let n0 = if b > n { n } else { (n / b) * b - 1 };
    let result = ((a * n) / b - a * (n / b)).max((a * n0) / b - a * (n0 / b));
    println!("{result}");
}
