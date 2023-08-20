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
        n: usize,
        a: [usize; n],
    }
    let result = a[1..].iter().fold(a[0], |acc, &x| acc * (x / gcd(acc, x)));
    println!("{}", result);
}
