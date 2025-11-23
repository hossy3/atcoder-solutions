use proconio::input;

fn main() {
    input! {
        s: usize,
        a: usize,
        b: usize,
        x: usize,
    }
    let result = s * (a * (x / (a + b)) + (x % (a + b)).min(a));
    println!("{result}");
}
