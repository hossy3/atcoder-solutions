use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut fact = 1;
    for i in 1..=n {
        fact *= i;
    }
    println!("{}", fact);
}
