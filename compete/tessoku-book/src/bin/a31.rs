use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let result = n / 3 + n / 5 - n / 15;
    println!("{}", result);
}
