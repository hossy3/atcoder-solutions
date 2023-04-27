use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    let result = if n > 0 { n / 10 } else { (n - 9) / 10 };
    println!("{}", result);
}
