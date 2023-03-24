use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }
    let mut sum = 0usize;
    let ab = a.iter().zip(b.iter());
    for (&a, &b) in ab {
        sum += a + b * 2;
    }
    let expected = (sum as f64) / 3.0;
    println!("{}", expected);
}
