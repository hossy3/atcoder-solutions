use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }

    let count = ab.iter().filter(|&&(a, b)| a > b).count();
    let sum = ab.iter().map(|&(a, b)| a.saturating_sub(b)).sum::<usize>();
    println!("{count} {sum}");
}
