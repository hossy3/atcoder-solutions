use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    for i in 0..n {
        let result = ((n - (k % n) + i) % n) + 1;
        println!("{result}");
    }
}
