use proconio::input;

fn main() {
    input! {
        n: isize,
        r: isize,
    }
    let result = r + 100 * (10 - n).max(0);
    println!("{result}");
}
