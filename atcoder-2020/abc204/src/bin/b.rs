use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let result: usize = a.iter().map(|&x| x - x.min(10)).sum();
    println!("{}", result);
}
