use proconio::input;

fn gcd(a: usize, b: usize) -> usize {
    if a % b == 0 {
        b
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
    }
    let result = n / x + n / y - n / (x * y / gcd(x, y));
    println!("{}", result);
}
