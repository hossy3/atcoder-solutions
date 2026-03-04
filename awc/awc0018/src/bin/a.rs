use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let result = 2 * (n - 1) * m;
    println!("{result}");
}
