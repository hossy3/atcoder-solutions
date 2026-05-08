use proconio::input;

fn lcm(a: u128, b: u128) -> u128 {
    fn gcd(a: u128, b: u128) -> u128 {
        if b == 0 { a } else { gcd(b, a % b) }
    }

    a / gcd(a, b) * b
}

fn f(m: u128, p: &[u128]) -> bool {
    let mut cur = 1u128;
    for &x in p {
        cur = lcm(cur, x);
        if cur > m {
            return false;
        }
    }

    cur <= m
}

fn main() {
    input! {
        n: usize,
        m: u128,
        p: [u128; n],
    }
    let yes = f(m, &p);
    println!("{}", if yes { "Yes" } else { "No" });
}
