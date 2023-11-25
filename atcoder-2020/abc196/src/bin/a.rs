use proconio::input;

fn main() {
    input! {
        _: i64,
        b: i64,
        c: i64,
        _: i64,
    }
    let result = b - c;
    println!("{}", result);
}
