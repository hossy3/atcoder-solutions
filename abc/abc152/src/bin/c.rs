use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }
    let mut min = usize::MAX;
    let mut result = 0usize;
    for &x in &p {
        if x <= min {
            result += 1;
            min = x;
        }
    }
    println!("{result}");
}
