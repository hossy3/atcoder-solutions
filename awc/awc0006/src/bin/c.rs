use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        d: usize,
        t: [usize; n],
    }
    let result = t
        .iter()
        .map(|&t| t.saturating_sub(m).div_ceil(d))
        .sum::<usize>();
    println!("{result}");
}
