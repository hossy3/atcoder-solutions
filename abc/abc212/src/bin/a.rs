use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let s = if a > 0 && b == 0 {
        "Gold"
    } else if a == 0 {
        "Silver"
    } else {
        "Alloy"
    };
    println!("{}", s);
}
