use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    }
    let z = if x == y { x } else { 3 - x - y };
    println!("{}", z);
}
