use proconio::input;

fn isqrt(n: usize) -> usize {
    let m = (n as f64).sqrt() as usize;
    if (m + 1).pow(2) <= n {
        m + 1
    } else if m.pow(2) <= n {
        m
    } else {
        m - 1
    }
}

fn main() {
    input! {
        n: usize,
    }

    let n = n / 2;
    let m = isqrt(n);
    let mut result = m;

    let n = n / 2;
    let m = isqrt(n);
    result += m;

    println!("{result}");
}
