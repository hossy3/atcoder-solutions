use proconio::input;

fn main() {
    input! {
        d: usize,
        f: usize,
    }
    let result = (((f - 1) + 700 - d) % 7) + 1;
    println!("{result}");
}
