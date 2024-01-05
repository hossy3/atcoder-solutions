use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let sum: usize = a.iter().sum();
    let ave0: usize = sum / n;
    let ave1 = if sum % n == 0 { ave0 } else { ave0 + 1 };
    let mut result0 = 0usize;
    let mut result1 = 0usize;
    for &x in &a {
        if x <= ave0 {
            result0 += ave0 - x;
        } else {
            result1 += x - ave1;
        }
    }
    let result = result0.max(result1);
    println!("{}", result);
}
