use proconio::input;

fn run(a: i64, b: i64, c: i64, x: i64) -> i64 {
    let s0 = (x / (a + c)) * a;
    let s1 = (x % (a + c)).min(a);
    (s0 + s1) * b
}

fn solve(a: i64, b: i64, c: i64, d: i64, e: i64, f: i64, x:i64) -> &'static str {
    let ta = run(a, b, c, x);
    let ao = run(d, e, f, x);
    if ta > ao {
        "Takahashi"
    } else if ta < ao {
        "Aoki"
    } else {
        "Draw"
    }
}

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
        e: i64,
        f: i64,
        x: i64,
    }
    let result = solve(a, b, c, d, e, f, x);
    println!("{}", result);
}
