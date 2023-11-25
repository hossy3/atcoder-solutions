use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let sum: i64 = a.iter().sum();
    let sum2: i64 = a.iter().map(|x| x * x).sum();
    let result = sum2 * (n as i64) - sum * sum;
    println!("{}", result);
}
