use proconio::input;

fn f(x: usize) -> usize {
    x / 100 + (x / 10) % 10 + x % 10
}

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let result = f(a).max(f(b));
    println!("{result}");
}
