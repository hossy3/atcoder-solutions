use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }
    let c = n / (a + b);
    let d = n % (a + b);
    let result = c * a + d.min(a);
    println!("{result}");
}
