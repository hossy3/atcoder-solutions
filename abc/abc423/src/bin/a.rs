use proconio::input;

fn main() {
    input! {
        x: usize,
        c: usize,
    }
    let result = x / (1000 + c) * 1000;
    println!("{result}");
}
