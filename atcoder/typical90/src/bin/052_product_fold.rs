use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        a: [[usize; 6]; n],
    }
    let b: Vec<usize> = a.iter().map(|x| x.iter().sum()).collect();
    let score = b.iter().fold(1, |acc, x| (acc * x) % MOD);
    println!("{score}");
}
