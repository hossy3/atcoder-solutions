use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let result = (n + 99) / 100;
    println!("{}", result);
}
