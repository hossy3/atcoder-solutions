use proconio::input;

fn main() {
    input! {
        x: usize,
    }
    let result = 100 - (x % 100);
    println!("{result}");
}
