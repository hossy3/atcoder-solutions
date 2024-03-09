use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let result = if a + b == 0 { 1 } else { 0 };
    println!("{result}");
}
