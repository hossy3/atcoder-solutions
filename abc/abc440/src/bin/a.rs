use proconio::input;

fn main() {
    input! {
        x: usize,
        y: u32,
    }
    let result = x * 2usize.pow(y);
    println!("{result}");
}
