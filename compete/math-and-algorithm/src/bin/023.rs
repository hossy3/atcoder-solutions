use proconio::input;

fn main() {
    input! {
        n: usize,
        b: [usize; n],
        r: [usize; n],
    }
    let sum = b.iter().sum::<usize>() + r.iter().sum::<usize>();
    let expected = (sum as f64) / (n as f64);
    println!("{}", expected);
}
