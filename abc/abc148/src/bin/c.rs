use proconio::input;

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let result = lcm(a, b);
    println!("{result}");
}
