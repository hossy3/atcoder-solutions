use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let result: usize = a
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, &x)| x)
        .sum();
    println!("{result}");
}
