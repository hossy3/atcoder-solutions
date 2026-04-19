use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize,
    }
    let result = r + 1 - l;
    println!("{result}");
}
