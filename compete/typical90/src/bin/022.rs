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
        c: usize,
    }
    let w = gcd(gcd(a, b), c);
    let count = (a / w - 1) + (b / w - 1) + (c / w - 1);
    println!("{}", count);
}
