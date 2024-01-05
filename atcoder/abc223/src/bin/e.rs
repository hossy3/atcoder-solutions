use proconio::input;

fn div_ceil(a: i64, b: i64) -> i64 {
    (a + b - 1) / b
}

fn g(x: i64, y: i64, a: i64, b: i64) -> bool {
    ((x > 0) && (y > 0))
        && ((div_ceil(a, x) + div_ceil(b, x) <= y) || (div_ceil(a, y) + div_ceil(b, y) <= x))
}

fn f(x: i64, y: i64, a: i64, b: i64, c: i64) -> bool {
    g(x, y - div_ceil(a, x), b, c)
        || g(x, y - div_ceil(b, x), a, c)
        || g(x, y - div_ceil(c, x), a, b)
        || g(x - div_ceil(a, y), y, b, c)
        || g(x - div_ceil(b, y), y, a, c)
        || g(x - div_ceil(c, y), y, a, b)
}

fn main() {
    input! {
        x: i64,
        y: i64,
        a: i64,
        b: i64,
        c: i64,
    }
    let yes = f(x, y, a, b, c);
    println!("{}", if yes { "Yes" } else { "No" });
}
