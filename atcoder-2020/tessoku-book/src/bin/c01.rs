use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let result = n + n / 10;
    println!("{}", result);
}
