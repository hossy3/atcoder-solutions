use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let n = n + if n >= 42 { 1 } else { 0 };
    println!("AGC{:03}", n);
}
