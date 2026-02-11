use proconio::input;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}
fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    // a/2 の最小公倍数を求める
    // mを超える時は条件を満たさないので抜ける
    let mut l = 1;
    for &x in &a {
        l = lcm(l, x / 2);
        if l > m {
            println!("0");
            return;
        }
    }

    // a/2 から最小公倍数にするとき偶数倍していると条件を満たさないので抜ける
    for &x in &a {
        if (l / (x / 2)) % 2 == 0 {
            println!("0");
            return;
        }
    }

    // m 以下の l の奇数倍が答え
    let result = (m / l).div_ceil(2);
    println!("{result}");
}
