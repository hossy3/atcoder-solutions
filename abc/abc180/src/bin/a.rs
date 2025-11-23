use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }
    let result = n - a + b;
    println!("{result}");
}
