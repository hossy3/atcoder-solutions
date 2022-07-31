use proconio::input;

fn main() {
    input! {
        y: usize,
    }
    println!("{}", ((y + 1) / 4) * 4 + 2);
}
