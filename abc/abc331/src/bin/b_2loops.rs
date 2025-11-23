use proconio::input;

fn f(n: i64, unit: i64) -> i64 {
    if n >= 0 {
        ((n as f64) / (unit as f64)).ceil() as i64
    } else {
        0
    }
}

const S: i64 = 6;
const M: i64 = 8;
const L: i64 = 12;

fn main() {
    input! {n: i64, s: i64, m: i64, l: i64, }
    let mut result = i64::MAX;
    let s_max = f(n, S);
    for s0 in 0..=s_max {
        let m_max = f(n - s0 * S, M);
        for m0 in 0..=m_max {
            let l0 = f(n - s0 * S - m0 * M, L);
            result = result.min(s * s0 + m * m0 + l * l0);
        }
    }
    println!("{result}");
}
