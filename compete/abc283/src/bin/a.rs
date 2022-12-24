use proconio::input;

fn main() {
    input! {
        a: usize,
        b: u32,
    }
    let result = a.pow(b);
    println!("{}", result);
}
