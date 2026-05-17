use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }
    let result = a.iter().map(|&a| a.saturating_sub(x)).sum::<usize>();
    println!("{result}");
}
