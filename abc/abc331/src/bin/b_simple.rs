use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        m: usize,
        l: usize,
    }
    let mut result = usize::MAX;
    for s0 in 0..=100 {
        for m0 in 0..=100 {
            for l0 in 0..=100 {
                if s0 * 6 + m0 * 8 + l0 * 12 >= n {
                    result = result.min(s * s0 + m * m0 + l * l0);
                }
            }
        }
    }
    println!("{result}");
}
