use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }
    let result: usize = ab.iter().map(|&(a, b)| (a + b) * (b - a + 1) / 2).sum();
    println!("{result}");
}
