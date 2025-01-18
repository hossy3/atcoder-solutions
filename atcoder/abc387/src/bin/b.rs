use proconio::input;

fn main() {
    input! {
        x: usize,
    }
    let result: usize = (1..=9)
        .map(|i| (1..=9).map(|j| i * j).filter(|&y| y != x).sum::<usize>())
        .sum();
    println!("{result}");
}
