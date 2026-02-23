use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        ab: [(usize, usize); n],
    }
    let result = ab
        .iter()
        .map(|&(a, b)| a.saturating_sub(b * t))
        .sum::<usize>();
    println!("{result}");
}
