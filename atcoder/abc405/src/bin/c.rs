use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut result = 0usize;
    let mut sum = 0usize;
    for &x in &a {
        result += sum * x;
        sum += x;
    }
    println!("{result}");
}
