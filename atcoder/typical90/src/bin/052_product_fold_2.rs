use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        a: [[usize; 6]; n],
    }
    let result = a
        .iter()
        .map(|x| x.iter().sum())
        .fold(1, |acc, x: usize| (acc * x) % MOD);
    println!("{result}");
}
