use proconio::input;

fn f(n: usize) -> f64 {
    if n == 1 {
        3.5
    } else if n < 3 {
        (6.0 + 5.0 + 4.0 + f(n - 1) * 3.0) / 6.0
    } else if n < 6 {
        (6.0 + 5.0 + f(n - 1) * 4.0) / 6.0
    } else {
        (6.0 + f(n - 1) * 5.0) / 6.0
    }
}

fn main() {
    input! {
        n: usize,
    }
    println!("{}", f(n));
}
