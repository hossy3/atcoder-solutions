use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    if a % b < b - (a % b) {
        println!("{}", a / b);
    } else {
        println!("{}", a / b + 1);
    }
}
