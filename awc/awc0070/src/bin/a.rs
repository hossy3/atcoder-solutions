use proconio::input;

fn main() {
    input! {
        n: usize,
        y: usize,
        m: usize,
        abpqc: [(usize, usize, usize, usize, usize); n],
    }

    let mut result0 = 0; // 当月
    let mut result1 = 0; // 翌月
    for &(_, _, p, q, c) in &abpqc {
        if y == p && m == q {
            result0 += c;
        } else if (m == 12 && y + 1 == p && q == 1) || (m < 12 && y == p && m + 1 == q) {
            result1 += c;
        }
    }
    println!("{result0} {result1}");
}
