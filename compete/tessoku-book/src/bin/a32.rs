use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }
    let c = n % (a + b);
    let first = (c >= b) || ((c / a) % 2 == 1);
    println!("{}", if first { "First" } else { "Second" });
}
