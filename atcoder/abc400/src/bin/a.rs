use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    if 400 % n == 0 {
        println!("{}", 400 / n);
    } else {
        println!("-1");
    }
}
