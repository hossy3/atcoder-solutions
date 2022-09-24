use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let score = a | b;
    println!("{}", score);
}
