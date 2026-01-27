use proconio::input;

fn main() {
    input! {
        x: usize,
    }
    let c500 = x / 500;
    let c5 = (x - c500 * 500) / 5;
    let result = c500 * 1000 + c5 * 5;
    println!("{result}");
}
