use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let result = if a + b >= 10 { 2 } else { 1 };
    println!("{result}");
}
