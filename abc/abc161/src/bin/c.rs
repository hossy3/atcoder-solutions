use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let x = n - (n / k) * k;
    let result = x.min(k - x);
    println!("{result}");
}
