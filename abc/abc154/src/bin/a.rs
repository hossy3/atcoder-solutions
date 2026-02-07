use proconio::input;

fn main() {
    input! {
        s: String,
        _: String,
        a: usize,
        b: usize,
        u: String,
    }
    if s == u {
        println!("{} {}", a - 1, b);
    } else {
        println!("{} {}", a, b - 1);
    }
}
