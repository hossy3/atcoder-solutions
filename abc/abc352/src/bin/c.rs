use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }
    let a_sum: usize = ab.iter().map(|&(a, _)| a).sum();
    let head_max = ab.iter().map(|&(a, b)| b - a).max().unwrap();
    let result = a_sum + head_max;
    println!("{result}");
}
