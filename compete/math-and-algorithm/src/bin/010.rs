use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let fact = (1..=n).fold(1, |acc, x| acc * x);
    println!("{}", fact);
}
