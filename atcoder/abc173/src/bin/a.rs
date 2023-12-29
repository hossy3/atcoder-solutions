use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let result = (10000 - n) % 1000;
    println!("{result}");
}
