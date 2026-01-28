use proconio::input;

fn main() {
    input! {
        n: isize,
        m: isize,
    }
    let result = n * (n - 1) / 2 + m * (m - 1) / 2;
    println!("{result}");
}
