use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
    }
    let a0 = a.iter().max().unwrap();
    let b0 = b.iter().min().unwrap();
    let result = (b0 - a0 + 1).max(0);
    println!("{}", result);
}
