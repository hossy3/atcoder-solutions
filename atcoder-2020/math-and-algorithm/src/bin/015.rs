use proconio::input;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let result = gcd(a, b);
    println!("{}", result);
}
