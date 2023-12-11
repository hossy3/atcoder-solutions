use proconio::input;

fn f(a: i64, b: i64, c: i64, d: i64) -> usize {
    if a == c && b == d {
        0
    } else if a + b == c + d || a - b == c - d || (a - c).abs() + (b - d).abs() <= 3 {
        1
    } else if (a + b).abs() % 2 == (c + d).abs() % 2
        || ((a + b) - (c + d)).abs() <= 3
        || ((a - b) - (c - d)).abs() <= 3
        || (a - c).abs() + (b - d).abs() <= 6
    {
        2
    } else {
        3
    }
}

fn main() {
    input! {
        r1: i64,
        c1: i64,
        r2: i64,
        c2: i64,
    }
    let result = f(r1, c1, r2, c2);
    println!("{result}");
}
