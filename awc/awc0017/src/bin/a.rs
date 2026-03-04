use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        cd: [(usize, usize); n],
    }
    let result: usize = cd.iter().filter(|&&(c, _)| c <= k).map(|&(_, d)| d).sum();
    println!("{result}");
}
