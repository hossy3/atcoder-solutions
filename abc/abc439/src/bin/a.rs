use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let x = 2usize;
    let result = x.pow(n as u32) - 2 * n;
    println!("{result}");
}
