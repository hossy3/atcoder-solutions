use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let result = (a + b - 1) / b;
    println!("{}", result);
}
