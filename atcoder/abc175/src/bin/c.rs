use proconio::input;

fn main() {
    input! {
        x: i64,
        k: i64,
        d: i64,
    }
    let result = if x.abs() / d >= k {
        x.abs() - k * d
    } else if (k - x.abs() / d) % 2 == 0 {
        x.abs() % d
    } else {
        d - (x.abs() % d)
    };
    println!("{result}");
}
