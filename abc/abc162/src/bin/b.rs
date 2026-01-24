use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let result: usize = (1..=n).filter(|&x| x % 3 > 0 && x % 5 > 0).sum();
    println!("{result}");
}
