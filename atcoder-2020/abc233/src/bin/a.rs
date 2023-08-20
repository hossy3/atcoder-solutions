use proconio::input;

fn main() {
    input! {
        x: i64,
        y: i64,
    }
    let result = (y - x + 9).max(0) / 10;
    println!("{}", result);
}
