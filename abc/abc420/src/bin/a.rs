use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    }
    let result = ((x - 1 + y) % 12) + 1;
    println!("{result}");
}
