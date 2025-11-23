use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    for i in 0..=n {
        for j in 0..=(n - i) {
            for k in 0..=(n - i - j) {
                println!("{i} {j} {k}");
            }
        }
    }
}
