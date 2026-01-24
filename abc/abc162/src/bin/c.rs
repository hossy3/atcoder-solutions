use proconio::input;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn main() {
    input! {
        k: usize,
    }
    let mut result = 0usize;
    for a in 1..=k {
        for b in 1..=k {
            let x = gcd(a, b);
            for c in 1..=k {
                result += gcd(x, c);
            }
        }
    }
    println!("{result}");
}
