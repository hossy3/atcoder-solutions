use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    let result = 21 - (a + b + c);
    println!("{}", result);
}
