use proconio::input;

fn main() {
    input! {
        mut n: usize,
        k: usize,
    }
    for _ in 0..k {
        n = if n % 200 == 0 {
            n / 200
        } else {
            n * 1000 + 200
        };
    }
    println!("{}", n);
}
