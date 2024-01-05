use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        x: usize,
        y: usize,
    }
    let result = if n <= a { x * n } else { x * a + y * (n - a) };
    println!("{}", result);
}
