use proconio::input;

fn main() {
    input! {
        n: usize,
        _: usize,
        _: usize,
        ab: [(usize, usize); n],
    }
    let nim = ab.iter().fold(0, |acc, (a, b)| acc ^ (a - 1) ^ (b - 1));
    let first = nim != 0;
    println!("{}", if first { "First" } else { "Second" });
}
