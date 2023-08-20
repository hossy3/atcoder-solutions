use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
        n: usize,
    }
    let score = if x * 3 > y {
        (n / 3) * y + (n % 3) * x
    } else {
        n * x
    };
    println!("{}", score);
}
