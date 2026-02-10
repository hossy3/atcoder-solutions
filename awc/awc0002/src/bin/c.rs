use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); n],
    }

    let result = ab
        .iter()
        .map(|&(a, b)| if a >= m { 0 } else { (m - a).div_ceil(b) })
        .max()
        .unwrap_or(0);
    println!("{result}");
}
