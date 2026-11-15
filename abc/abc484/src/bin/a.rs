use proconio::input;

fn main() {
    input! {
        b: usize,
        r: usize,
    }
    let result = (30 - b) * (30 - r);
    println!("{result}");
}
