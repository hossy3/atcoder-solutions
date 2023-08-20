use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    }
    let result = (b - a + 1).max(0);
    println!("{}", result);
}
