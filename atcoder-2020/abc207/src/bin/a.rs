use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    let result = (a + b).max(b + c).max(c + a);
    println!("{}", result);
}
