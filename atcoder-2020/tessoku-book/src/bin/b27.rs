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
        a: usize,
        b: usize,
    }
    let result = a * b / gcd(a, b);
    println!("{}", result);
}
