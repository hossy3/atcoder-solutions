use proconio::input;

fn f(x: i64, y: i64, z: i64) -> i64 {
    if x * y < 0 || x.abs() < y.abs()  {
        return x.abs();
    }
    if x * z < 0 {
        return x.abs() + z.abs() * 2;
    }
    if z.abs() < y.abs() {
        return x.abs();
    }
    -1
}

fn main() {
    input! {
        x: i64,
        y: i64,
        z: i64,
    }
    let score = f(x, y, z);
    println!("{}", score);
}
