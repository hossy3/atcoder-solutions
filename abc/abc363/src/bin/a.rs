use proconio::input;

fn main() {
    input! {
        r: usize,
    }
    let result = 100 - r % 100;
    println!("{result}");
}
