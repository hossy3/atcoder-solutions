use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let result = 2 * a + 100 - b;
    println!("{result}");
}
