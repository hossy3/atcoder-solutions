use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let result = a[1..].iter().fold(a[0], |acc, x| (acc + x) % 100);
    println!("{}", result);
}
