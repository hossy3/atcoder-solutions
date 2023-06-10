use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let m = match n % 5 {
        0 => n,
        1 => n - 1,
        2 => n - 2,
        3 => n + 2,
        _ => n + 1
    };
    println!("{}", m);
}
