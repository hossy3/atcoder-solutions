use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let result = n + 1 - k;
    println!("{result}");
}
