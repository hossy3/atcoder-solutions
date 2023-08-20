use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: usize,
    }
    let result = (((a - 1) + (k - 1)) % n) + 1;
    println!("{}", result);
}
