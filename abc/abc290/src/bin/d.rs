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
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            d: usize,
            k: usize,
        }
        let x = n / gcd(n, d);
        let a = (((k - 1) / x) + ((k - 1) % x) * d) % n;
        println!("{}", a);
    }
}
