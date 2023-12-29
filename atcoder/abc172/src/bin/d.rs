use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut result = 0usize;
    for i in 1..=n {
        result += (n / i) * (n / i + 1) / 2 * i;
    }
    println!("{result}");
}
