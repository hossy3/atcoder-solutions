use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let result: usize = (1..=n)
        .map(|i| (1..=k).map(|j| i * 100 + j).sum::<usize>())
        .sum();
    println!("{}", result);
}
