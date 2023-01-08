use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    // see: https://doc.rust-lang.org/std/fmt/index.html
    println!("{:010b}", n);
}
