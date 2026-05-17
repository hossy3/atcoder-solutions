use proconio::input;

fn lcm(a: usize, b: usize) -> usize {
    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 { a } else { gcd(b, a % b) }
    }
    a / gcd(a, b) * b
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn main() {
    input! {
        n: usize,
        pq: [(usize, usize); n],
    }

    let (mut p0, mut q0) = pq[0];
    for &(p, q) in &pq[1..] {
        p0 = lcm(p0, p);
        q0 = gcd(q0, q);
    }
    println!("{p0} {q0}");
}
