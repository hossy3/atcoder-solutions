use proconio::input;

fn f(mut x: usize) -> usize {
    let mut result = 0;
    while x > 0 {
        result = result * 10 + x % 10;
        x /= 10;
    }
    result
}

fn main() {
    input! {
        mut x: usize,
        mut y: usize,
    }
    for _ in 0..8 {
        (x, y) = (y, f(x + y));
    }
    println!("{y}");
}
