use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    for i in 0..n {
        let result = m / n + if m % n > i { 1 } else { 0 };
        println!("{}", result);
    }
}
